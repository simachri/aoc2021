use std::env;
use std::fs;

fn main() {
    let fname: &str = &env::args().skip(1).take(1).last().unwrap();
    let res1 = rust_day4::run1(&fs::read_to_string(fname).unwrap()).unwrap();
    println!("Result of Day 4, Part 1: {}", res1);
    let res2 = rust_day4::run2(&fs::read_to_string(fname).unwrap()).unwrap();
    println!("Result of Day 4, Part 2: {}", res2);
}
