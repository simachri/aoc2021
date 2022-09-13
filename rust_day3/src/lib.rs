#![feature(drain_filter)]
use std::error::Error;

pub fn run1(input: &str) -> Result<u32, Box<dyn Error>> {
    let line_len = input.split_terminator("\n").next().unwrap().trim().len();

    let bit_count_per_line_pos = input
        .lines()
        // map each line into a vector of 0 and 1
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        // for each line: count the amount of 0 and 1 of each position
        .fold(
            (0..line_len).map(|_| (0, 0)).collect(),
            |mut count_of_0_and_1: Vec<(i32, i32)>, line_bin| {
                //|mut bit_count_per_line_pos: Vec<(u8, u8)>, line_bin| {
                for (idx, bin_num) in line_bin.iter().enumerate() {
                    count_of_0_and_1 = match bin_num {
                        0 => {
                            count_of_0_and_1[idx].0 += 1;
                            count_of_0_and_1
                        }
                        1 => {
                            count_of_0_and_1[idx].1 += 1;
                            count_of_0_and_1
                        }
                        _ => unreachable!(),
                    };
                }
                count_of_0_and_1
            },
        );

    let mut gamma_vec: Vec<u8> = (0..line_len).map(|_| 0).collect();
    let mut epsilon_vec: Vec<u8> = (0..line_len).map(|_| 0).collect();
    for (idx, (count_0, count_1)) in bit_count_per_line_pos.iter().enumerate() {
        if count_1 > count_0 {
            gamma_vec[idx] = 1;
            epsilon_vec[idx] = 0;
        } else {
            gamma_vec[idx] = 0;
            epsilon_vec[idx] = 1;
        }
    }

    let gamma: String = gamma_vec
        .iter()
        .map(|num| num.to_string())
        .reduce(|mut accum, itm| {
            accum.push_str(&itm);
            accum
        })
        .unwrap();
    let epsilon: String = epsilon_vec
        .iter()
        .map(|num| num.to_string())
        .reduce(|mut accum, itm| {
            accum.push_str(&itm);
            accum
        })
        .unwrap();

    Ok(u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap())
}

pub fn run2(input: &str) -> Result<u32, Box<dyn Error>> {
    let line_len = input.split_terminator("\n").next().unwrap().trim().len();

    let nums = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let oxy = (0..line_len)
        // reverse direction is used because this makes the bitwise left-shift operator (see value 
        // i below) start with the highest value, which tests the most-left bit first and then 
        // traverses from left to right.
        .rev()
        .inspect(|el| println!("rev: Current element is {:?}", el))
        .scan(nums.clone(), |oxy, i| {
            println!("scan: oxy will be reduced. It currently is:");
            oxy.iter().for_each(|o| println!("{:b}", o));
            println!("scan: i, which is the positional indicator of 0..line_len is {}", i);
            println!("scan: For the current set of oxy, check if '1' at position {} (line_len - i - 1) is the most significant bit.", line_len - i - 1);
            let one = oxy.iter().filter(|n| {
                println!("scan: n is {:b}", n);
                println!("scan: *n & 1 << i is {:b}", *n & 1 << i);
                println!("scan: *n & 1 << i evaluates to {:?}", *n & 1 << i > 0);
                *n & 1 << i > 0
            }).count() >= (oxy.len() + 1) / 2;
            println!("scan: one (1) is most significant bit at position {}: {:?}", line_len - i - 1, one);
            println!("scan: Remove all entries from oxy that do not have 1 at position {}.", line_len - i - 1);
            oxy.drain_filter(|n| (*n & 1 << i > 0) != one);
            println!("scan: oxy.first().copied() is {:b}.", oxy.first().copied().unwrap());
            oxy.first().copied()
            //None
        })
        .inspect(|el| println!("after scan: Current element is {:b}", el))
        .last()
        .unwrap();

    let co2 = (0..line_len)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            co2.drain_filter(|n| (*n & 1 << i > 0) == one);
            co2.first().copied()
        })
        .last()
        .unwrap();

    Ok(oxy * co2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_part_1_isok() {
        let input = "\
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010";
        match run1(&input) {
            Ok(got) => {
                const WANT: u32 = 198;
                assert_eq!(WANT, got)
            }
            Err(err) => {
                panic!("Test failed. Error returned: {}", err)
            }
        }
    }

    #[test]
    fn result_part_2_isok() {
        let input = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        match run2(&input) {
            Ok(got) => {
                const WANT: u32 = 198;
                assert_eq!(WANT, got)
            }
            Err(err) => {
                panic!("Test failed. Error returned: {}", err)
            }
        }
    }
}
