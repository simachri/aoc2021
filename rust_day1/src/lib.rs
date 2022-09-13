use std::error::Error;
use std::fs;

pub fn run(conf: &Config) -> Result<u32, Box<dyn Error>> {
    let contents = fs::read_to_string(conf.input_file)?;
    Ok(calculate_increase_count(&contents)?)
}

fn calculate_increase_count(input: &str) -> Result<u32, Box<dyn Error>> {
    let mut increase_count = 0;

    let mut win_1: [u32; 3] = [0; 3];
    let mut win_2: [u32; 3] = [0; 3];
    let mut win_3: [u32; 3] = [0; 3];
    let mut win_1_idx: usize = 0;
    let mut win_2_idx: usize = 0;
    let mut win_3_idx: usize = 0;
    let mut win_1_init = true;
    let mut win_2_init = true;
    let mut full_windows_sum: [u32; 2] = [0, 0];
    let mut full_windows_sum_idx: usize = 0;

    for val in input.lines() {
        let mut sum = 0;
        let val_i = val.trim().parse::<u32>()?;

        win_1[win_1_idx] = val_i;
        win_1_idx += 1;
        if win_1_init {
            win_1_init = false;
            continue;
        }
        if win_1_idx == 3 {
            for val in win_1.iter() {
                sum += val;
            }
            full_windows_sum[full_windows_sum_idx] = sum;
            win_1_idx = 0;
        }

        win_2[win_2_idx] = val_i;
        win_2_idx += 1;
        if win_2_init {
            win_2_init = false;
            continue;
        }
        if win_2_idx == 3 {
            for val in win_2.iter() {
                sum += val;
            }
            full_windows_sum[full_windows_sum_idx] = sum;
            win_2_idx = 0;
        }

        win_3[win_3_idx] = val_i;
        win_3_idx += 1;
        if win_3_idx == 3 {
            for val in win_3.iter() {
                sum += val;
            }
            full_windows_sum[full_windows_sum_idx] = sum;
            win_3_idx = 0;
        }

        if full_windows_sum_idx == 0 {
            full_windows_sum_idx = 1;
            continue;
        }

        if full_windows_sum[1] > full_windows_sum[0] {
            increase_count += 1;
        }
        full_windows_sum[0] = full_windows_sum[1];
    }

    Ok(increase_count)
}

pub struct Config<'a> {
    input_file: &'a String,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("Expected exactly one argument.");
        }
        Ok(Config {
            input_file: &args[1],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_demo_is_7() -> Result<(), Box<dyn Error>> {
        let input = "\
199
200
208
210
200
207
240
269
260
263";
        match calculate_increase_count(&input) {
            Err(why) => return Err(format!("Error raised: {}", why).into()),
            Ok(count) => {
                const WANT: u32 = 5;
                if count == WANT {
                    return Ok(());
                } else {
                    return Err(format!("Want {}, got {}", WANT, count).into());
                }
            }
        }
    }
}
