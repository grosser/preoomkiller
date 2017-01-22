extern crate libc;
extern crate getopts;
extern crate regex;
use std::fs::File;
use std::io::Read;

fn do_work(args: Vec<String>, max_path: String, used_path: String) {
    let mut child = std::process::Command::new(&args[0]).
        args(&args[1..]).
        spawn().
        expect("Failed to start");
    let child_id = child.id();

    // open channel so we can communicate with our watcher
    let (tx, rx) = std::sync::mpsc::channel();

    let memory_watcher = std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));

            // used is a single number of bytes
            let used = parse_int(&read_file(&used_path));

            // max is bytes after hierarchical_memory_limit
            let max = {
                let max_text = &capture(&read_file(&max_path), r"hierarchical_memory_limit\s+(\d+)", 1);
                parse_int(max_text)
            };

            if used > max {
                unsafe {
                    libc::kill(child_id as i32, libc::SIGTERM);
                }
            }

            // TODO: wrap in a method
            match rx.try_recv() {
                // Child is done or process failed ... time to stop
                Ok(_) | Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    println!("Terminating.");
                    break;
                }
                // Child is not done ... continue
                Err(std::sync::mpsc::TryRecvError::Empty) => {
                    println!("Nothing.");
                }
            }
        }
    });

    // wait for the command to finish
    child.wait().expect("failed to wait");

    // tell the watcher to stop
    let _ = tx.send(());

    memory_watcher.join().expect("joining memory_inspector fail");
}

fn parse_int(string: &String) -> i32 {
    string.trim().parse().unwrap()
}

fn capture(string: &String, pattern: &str, index: usize) -> String {
    let regex = regex::Regex::new(pattern).unwrap();
    regex.captures(&string).unwrap().get(index).unwrap().as_str().to_string()
}

// TODO: .expect(&format!("Could not read {}", file));
fn read_file(path: &String) -> String {
    let mut data = String::new();
    let mut file = File::open(path).expect("Unable to open file");
    file.read_to_string(&mut data).expect("Unable to read string");
    data
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let program = args.remove(0);

    let mut opts = getopts::Options::new();
    opts.optopt("m", "max-memory-file", "set file to read maximum memory from", "PATH");
    opts.optopt("u", "used-memory-file", "set file to read used memory from", "PATH");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args) { // TODO: use unwrap_or_else or expect
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // show usage when run without argv
    if matches.free.is_empty() {
        print_usage(&program, opts);
        return
    }

    // User wants help
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return
    }

    // Parse max-memory file location or use default
    let max_memory_file = matches.opt_str("max-memory-file").
        unwrap_or_else(|| "/sys/fs/cgroup/memory/memory.stat".to_string());

    // Parse used-memory file location or use default
    let used_memory_file = matches.opt_str("used-memory-file").
        unwrap_or_else(|| "/sys/fs/cgroup/memory/memory.usage_in_bytes".to_string());

    do_work(matches.free, max_memory_file, used_memory_file);
}
