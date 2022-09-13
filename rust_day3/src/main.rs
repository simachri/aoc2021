use std::env;
use std::fs;

fn main() {
    let result1 =
        rust_day3::run1(&fs::read_to_string(&env::args().collect::<Vec<String>>()[1]).unwrap())
            .unwrap();
    println!("Day 3, part 1 - Result is: {}", result1);

    let result2 =
        rust_day3::run2(&fs::read_to_string(&env::args().collect::<Vec<String>>()[1]).unwrap())
            .unwrap();
    println!("Day 3, part 2 - Result is: {}", result2);
}
