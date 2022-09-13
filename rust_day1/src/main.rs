use std::env;
use std::process;

use aoc::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::build(&args).unwrap_or_else(|err| {
        println!("Failed to parse commandline arguments: {err}");
        process::exit(1);
    });

    match aoc::run(&conf) {
        Err(e) => {
            println!("Application error: {e}");
            process::exit(1);
        },
        Ok(v) => {
            println!("Result is '{}'.", v);
        }
    }
}
