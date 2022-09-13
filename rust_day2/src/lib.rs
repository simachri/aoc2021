use std::error::Error;

pub fn run_part1(contents: &str) -> Result<u32, Box<dyn Error>> {
    let (final_pos_x, final_pos_y) = contents.lines().map(|el| el.split_once(' ').unwrap()).fold(
        (0, 0),
        |(pos_x, pos_y), (nav_cmd, nav_amount)| match (nav_cmd, nav_amount.parse::<u32>().unwrap())
        {
            ("forward", nav_amount) => (pos_x + nav_amount, pos_y),
            ("down", nav_amount) => (pos_x, pos_y + nav_amount),
            ("up", nav_amount) => (pos_x, pos_y - nav_amount),
            _ => unreachable!(),
        },
    );
    Ok(final_pos_x * final_pos_y)
    //let mut pos = (0, 0);

    //for line in contents.lines() {
    //if line.starts_with("forward") {
    //pos.1 += str::parse::<u32>(line.split(' ').collect::<Vec<&str>>()[1])?;
    //}
    //else if line.starts_with("down") {
    //pos.0 += str::parse::<u32>(line.split(' ').collect::<Vec<&str>>()[1])?;
    //}
    //else if line.starts_with("up") {
    //pos.0 -= str::parse::<u32>(line.split(' ').collect::<Vec<&str>>()[1])?;
    //}
    //}
    //Ok(pos.0 * pos.1)
}

pub fn run_part2(contents: &str) -> Result<u32, Box<dyn Error>> {
    let (pos_x, pos_y, _) = contents.lines().map(|l| l.split_once(" ").unwrap()).fold(
        (0, 0, 0),
        |(pos_x, pos_y, aim), (cmd, amount)| match (cmd, amount.parse::<u32>().unwrap()) {
            ("forward", amount) => (pos_x + amount, pos_y + aim * amount, aim),
            ("down", amount) => (pos_x, pos_y, aim + amount),
            ("up", amount) => (pos_x, pos_y, aim - amount),
            _ => unreachable!(),
        },
    );
    Ok(pos_x * pos_y)
}

#[test]
fn day2p1_demo_data_passes() -> Result<(), Box<dyn Error>> {
    let input = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";
    match run_part1(&input) {
        Err(why) => return Err(format!("Error raised: {}", why).into()),
        Ok(count) => {
            const WANT: u32 = 150;
            if count == WANT {
                return Ok(());
            } else {
                return Err(format!("Want {}, got {}", WANT, count).into());
            }
        }
    }
}

#[test]
fn day2p2_demo_data_passes() -> Result<(), Box<dyn Error>> {
    let input = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";
    match run_part2(&input) {
        Err(why) => return Err(format!("Error raised: {}", why).into()),
        Ok(count) => {
            const WANT: u32 = 900;
            if count == WANT {
                return Ok(());
            } else {
                return Err(format!("Want {}, got {}", WANT, count).into());
            }
        }
    }
}
