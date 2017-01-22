Softly kills your process with SIGTERM before it runs out of memory.

Written in Rust to consume minimal resources.

Tested with Ruby to have readable / expressive tests.

Version: 0.0.1

### Usage

<!-- Updated by rake bump:patch -->
```
Usage: target/debug/preoomkiller [options] args

Options:
    -m, --max-memory-file PATH
                        set file to read maximum memory from
    -u, --used-memory-file PATH
                        set file to read used memory from
    -i, --interval SECONDS
                        how often to check memory usage
    -p, --percent PERCENT
                        maximum memory usage percent
    -h, --help          print this help menu
    -v, --version       show version
```
<!-- Updated by rake bump:patch -->

### Build
 - [install rust](https://www.rust-lang.org/en-US/install.html)
 - `cargo build`

### Test
 - Build
 - `gem install bundler` ... needs [ruby](https://www.ruby-lang.org/en/) installed
 - `bundle`
 - `bundle exec rake`
 
### Release
 - `bundle exec rake bump:patch`
 - `cargo build --release`
 - take `target/release/preoomkiller` binary

### TODO
 - kill child when process is killed (already has a failing test)
 - find safe way of doing wait / kill ... http://stackoverflow.com/questions/35093869
 - make `rake bump` release for multiple targets and commit all changes
 - add `--signal` option ... support numbers and ideally `USR1` etc words
 - make percent a float
 - add `--restart` option to not kill but restart ... maybe don't since this is tricky / needs a limit
