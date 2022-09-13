use std::env;
use std::fs;
use std::process;

use rust_day2;

fn main() {
    let input = fs::read_to_string(&env::args().collect::<Vec<String>>()[1]).unwrap_or_else(|e| {
        println!("Error: {}", e);
        process::exit(1);
    });

    let result_part1 = rust_day2::run_part1(&input).unwrap_or_else(|e| {
        println!("Error: {}", e);
        process::exit(1);
    });
    println!("Result for part 1 is: {}", result_part1);

    let result_part2 = rust_day2::run_part2(&input).unwrap_or_else(|e| {
        println!("Error: {}", e);
        process::exit(1);
    });
    println!("Result for part 2 is: {}", result_part2);
}
