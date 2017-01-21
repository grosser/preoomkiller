extern crate getopts;

use getopts::Options;
use std::env;

fn do_work(inp: &Vec<String>, max: String, used: String) {
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
        return
    }

    // User wants help
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return
    }

    // Parse max-memory file location or use default
    let mut max_memory_file = matches.opt_str("max-memory-file").
        unwrap_or_else(|| "/sys/fs/cgroup/memory/memory.stat".to_string());

    // Parse used-memory file location or use default
    let mut used_memory_file = matches.opt_str("used-memory-file").
        unwrap_or_else(|| "/sys/fs/cgroup/memory/memory.usage_in_bytes".to_string());

    do_work(&matches.free, max_memory_file, used_memory_file)
}
