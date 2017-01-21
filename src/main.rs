extern crate getopts;
use std::fs::File;
use std::io::Read;

fn do_work(args: Vec<String>, max: String, used: String) {
    let mut child = std::process::Command::new(&args[0]).
        args(&args[1..]).
        spawn().
        expect("Failed to start");

    // open channel so we can communicate with our watcher
    let (tx, rx) = std::sync::mpsc::channel();

    let memory_watcher = std::thread::spawn(move || {
        loop {
            std::thread::sleep_ms(1_000);

            read_file(&max);
            read_file(&used);

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

fn read_file(path: &String) -> String {
    let mut data = String::new();
    let mut file = File::open(path).expect("Unable to open file"); // TODO: tell user what file was missing
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
