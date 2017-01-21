extern crate getopts;

use getopts::Options;
use std::env;

fn do_work(inp: &Vec<String>) {
    println!("{:?}", inp);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let program = args.remove(0);

    let mut opts = Options::new();
    opts.optopt("m", "max-memory-file", "set file to read maximum memory from", "PATH");
    opts.optopt("u", "used-memory-file", "set file to read used memory from", "PATH");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // show usage when run without argv
    if matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    }

    // User wants help
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // Parse max-memory file location or use default
    let mut max_memory_file = "/sys/fs/cgroup/memory/memory.stat".to_string();
    let max_options = matches.opt_str("max-memory-file");
    if max_options.is_some() {
        max_memory_file = max_options.unwrap();
    }

    // Parse used-memory file location or use default
    let mut used_memory_file = "/sys/fs/cgroup/memory/memory.usage_in_bytes".to_string();
    let used_options = matches.opt_str("used-memory-file");
    if used_options.is_some() {
        used_memory_file = used_options.unwrap();
    }

    do_work(&matches.free);
}
