use std::env;
use std::fs;

fn main() {
    let input = &env::args().skip(1).take(1).last().unwrap();
    let res1 = rust_day5::run1(&fs::read_to_string(input).unwrap(), false).unwrap(); 
    println!("Result of day 5, part 1: {}", res1);
    let res2 = rust_day5::run1(&fs::read_to_string(input).unwrap(), true).unwrap(); 
    println!("Result of day 5, part 2: {}", res2);

}
