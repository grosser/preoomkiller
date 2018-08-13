extern crate libc;
extern crate getopts;
extern crate regex;
use std::fs::File;
use std::io::Read;
use std::io::Write;

macro_rules! abort(
    ($($arg:tt)*) => { {
        writeln!(&mut std::io::stderr(), $($arg)*).expect("failed printing to stderr");
        std::process::exit(1)
    } }
);

fn do_work(args: Vec<String>, max_path: String, used_path: String, interval: u64, max_usage_percent: u64) {
    // read both files once to make sure they exist before we start our child
    read_file(&used_path);
    read_file(&max_path);

    // start monitored child process
    let mut child = std::process::Command::new(&args[0]).
        args(&args[1..]).
        spawn().
        expect("Failed to start");
    let child_id = child.id();

    // open channel so we can communicate with our watcher
    let (tx, rx) = std::sync::mpsc::channel();

    let memory_watcher = std::thread::spawn(move || {
        loop {
            // If the child is done we can stop and do not need to do any checks
            match rx.recv_timeout(std::time::Duration::from_millis(interval)) {
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => { }
                Ok(_) | Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => { break }
            }

            // used is a single number of bytes
            let used = parse_int(&read_file(&used_path));

            // max is bytes after hierarchical_memory_limit adjusted by what the user deems safe
            let max_allowed = {
                let system_max = parse_int(&capture(&read_file(&max_path), r"hierarchical_memory_limit\s+(\d+)", 1));
                (system_max / 100) * max_usage_percent
            };

            if used > max_allowed {
                unsafe { libc::kill(child_id as i32, libc::SIGTERM); }
                println!("Terminated by preoomkiller"); // TODO: write to stderr
                std::process::exit(1)
            }
        }
    });

    // wait for the command to finish
    child.wait().expect("failed to wait");

    // tell the watcher to stop
    tx.send(()).expect("Unable to send to child");

    memory_watcher.join().expect("joining memory_inspector fail");
}

fn parse_int(string: &String) -> u64 {
    string.trim().parse::<u64>().unwrap()
}

fn capture(string: &String, pattern: &str, index: usize) -> String {
    let regex = regex::Regex::new(pattern).unwrap();
    regex.captures(&string).unwrap().get(index).unwrap().as_str().to_string()
}

fn read_file(path: &String) -> String {
    let mut data = String::new();
    let mut file = File::open(path).unwrap_or_else(|_| { abort!("Could not open {}", path) });
    file.read_to_string(&mut data).expect("Unable to read string");
    data
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options] args", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let program = args.remove(0);

    let mut opts = getopts::Options::new();
    opts.parsing_style(getopts::ParsingStyle::StopAtFirstFree);
    opts.optopt("m", "max-memory-file", "set file to read maximum memory from", "PATH");
    opts.optopt("u", "used-memory-file", "set file to read used memory from", "PATH");
    opts.optopt("i", "interval", "how often to check memory usage", "SECONDS");
    opts.optopt("p", "percent", "maximum memory usage percent", "PERCENT"); // TODO: float support
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "show version");

    let matches = match opts.parse(&args) { // TODO: use unwrap_or_else or expect
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // User wants help
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return
    }

    // User wants help
    if matches.opt_present("v") {
        println!("0.0.2"); // modified via `rake bump:patch`
        return
    }

    // show usage when run without argv
    if matches.free.is_empty() {
        print_usage(&program, opts);
        std::process::exit(1)
    }

    // Parse max-memory file location or use default
    let max_memory_file = matches.opt_str("max-memory-file").
        unwrap_or_else(|| "/sys/fs/cgroup/memory/memory.stat".to_string());

    // Parse used-memory file location or use default
    let used_memory_file = matches.opt_str("used-memory-file").
        unwrap_or_else(|| "/sys/fs/cgroup/memory/memory.usage_in_bytes".to_string());

    // Parse interval to milliseconds
    let interval = {
        let raw_interval:f64 = matches.opt_str("interval").unwrap_or_else(|| "1".to_string()).parse().unwrap();
        (raw_interval * 1000.0).round() as u64
    };

    // Parse max usage percent to integer
    let max_usage_percent:u64 = matches.opt_str("percent").unwrap_or_else(|| "90".to_string()).parse::<u64>().unwrap();
    if max_usage_percent >= 100 {
        abort!("Using >= 100 percent of memory will never happen since the process would already be OOM")
    }

    do_work(matches.free, max_memory_file, used_memory_file, interval, max_usage_percent);
}
