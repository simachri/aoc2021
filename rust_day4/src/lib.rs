#![feature(drain_filter)]
use std::collections::HashSet;
use std::error::Error;

#[derive(Default, Debug, Clone)]
struct Board {
    numbers: [[i32; 5]; 5],
    matched_numbers: HashSet<i32>,
    cols_hit_count: [i32; 5],
    rows_hit_count: [i32; 5],
    bingo: bool,
}

fn parse_boards<'a, I>(boards_in: I, first_five_drawn_numbers: &[i32]) -> Vec<Board>
where
    I: Iterator<Item = &'a str>,
{
    let mut boards: Vec<Board> = Vec::new();
    for board in boards_in {
        if board == "" {
            continue;
        }
        println!("Board: {:?}", board);
        boards.push(board.lines().enumerate().fold(
            Board::default(),
            |mut board, (row_idx, row)| {
                row.split_whitespace()
                    .enumerate()
                    .map(|(col_idx, num_str)| {
                        let num = num_str.parse::<i32>().unwrap();
                        board.numbers[row_idx][col_idx] = num;
                        (col_idx, num)
                    })
                    .filter(|(_, num)| first_five_drawn_numbers.contains(num))
                    .for_each(|(col_idx, hit)| {
                        board.cols_hit_count[col_idx] += 1;
                        board.rows_hit_count[row_idx] += 1;
                        board.matched_numbers.insert(hit);
                    });
                board.clone()
            },
        ));
    }
    boards
}

pub fn run1(content: &str) -> Result<i32, Box<dyn Error>> {
    let drawn_numbers: Vec<i32> = content
        .lines()
        .take(1)
        .next()
        .unwrap()
        .split(',')
        .map(|el| el.parse::<i32>().unwrap())
        .collect();

    let content_without_first_two_lines = content.lines().skip(2).collect::<Vec<_>>().join("\n");

    let boards_raw = content_without_first_two_lines.split("\n\n");

    let first_five_drawn_numbers: [i32; 5] = drawn_numbers
        .iter()
        .take(5)
        .map(|num_ptr| *num_ptr)
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap();

    let mut boards = parse_boards(boards_raw, &first_five_drawn_numbers);

    let mut result = -1;
    let mut is_hit = false;
    let mut bingo = false;
    for drawn_number in drawn_numbers.iter().skip(5) {
        println!("Drawn number is {}", *drawn_number);
        for board in boards.iter_mut() {
            for (row_idx, row) in board.numbers.iter().enumerate() {
                for (col_idx, num) in row.iter().enumerate() {
                    if *num == *drawn_number {
                        is_hit = true;
                        board.cols_hit_count[col_idx] += 1;
                        board.matched_numbers.insert(*drawn_number);
                    }
                }
                if is_hit {
                    board.rows_hit_count[row_idx] += 1;
                    is_hit = false;
                }
                if board.rows_hit_count.contains(&5) || board.cols_hit_count.contains(&5) {
                    println!("Bingo! {:?}", board);
                    bingo = true;
                    break;
                }
            }
            if bingo {
                let sum_of_unmarked_numbers = board
                    .numbers
                    .iter()
                    .map::<i32, _>(|row| {
                        row.iter()
                            .filter(|num| !board.matched_numbers.contains(*num))
                            .sum()
                    })
                    .sum::<i32>();
                println!("Sum of unmarked numbers: {}", sum_of_unmarked_numbers);
                result = sum_of_unmarked_numbers * *drawn_number;
                break;
            }
        }
        if bingo {
            break;
        }
    }

    Ok(result)
}

pub fn run2(content: &str) -> Result<i32, Box<dyn Error>> {
    let drawn_numbers: Vec<i32> = content
        .lines()
        .take(1)
        .next()
        .unwrap()
        .split(',')
        .map(|el| el.parse::<i32>().unwrap())
        .collect();

    let content_without_first_two_lines = content.lines().skip(2).collect::<Vec<_>>().join("\n");

    let boards_raw = content_without_first_two_lines.split("\n\n");

    let first_five_drawn_numbers: [i32; 5] = drawn_numbers
        .iter()
        .take(5)
        .map(|num_ptr| *num_ptr)
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap();

    let mut boards = parse_boards(boards_raw, &first_five_drawn_numbers);

    let mut result = -1;
    let mut is_hit = false;
    let mut is_bingo = false;
    for drawn_number in drawn_numbers.iter().skip(5) {
        println!("Drawn number is {}", *drawn_number);
        for board in boards.iter_mut() {
            for (row_idx, row) in board.numbers.iter().enumerate() {
                for (col_idx, num) in row.iter().enumerate() {
                    if *num == *drawn_number {
                        is_hit = true;
                        board.cols_hit_count[col_idx] += 1;
                        board.matched_numbers.insert(*drawn_number);
                    }
                }
                if is_hit {
                    board.rows_hit_count[row_idx] += 1;
                    is_hit = false;
                }
                if board.rows_hit_count.contains(&5) || board.cols_hit_count.contains(&5) {
                    println!("Bingo! This is the board:\n{:?}", board);
                    board.bingo = true;
                    is_bingo = true;
                    break;
                }
            }
        }

        if is_bingo {
            match boards.len() {
                1 => {
                    println!("Last Bingo! board found.");
                    let sum_of_unmarked_numbers = boards[0]
                        .numbers
                        .iter()
                        .map::<i32, _>(|row| {
                            row.iter()
                                .filter(|num| !boards[0].matched_numbers.contains(*num))
                                .sum()
                        })
                        .sum::<i32>();
                    println!("Sum of unmarked numbers: {}", sum_of_unmarked_numbers);
                    result = sum_of_unmarked_numbers * *drawn_number;
                    break;
                }
                _ => {
                    boards.drain_filter(|board| {
                        println!("Removing a Bingo! board from the set of boards.");
                        board.bingo
                    });
                }
            }
            is_bingo = false;
        }

        println!("Remaining boards count: {}", boards.len());
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_boards1_isok() {
        let input = "

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6";
        let boards = parse_boards(input.split("\n\n"), &[8, 0, 6]);
        assert_eq!(2, boards.len());
        assert_eq!([2, 0, 0, 0, 1], boards.get(0).unwrap().cols_hit_count);
        assert_eq!([1, 1, 0, 1, 0], boards.get(0).unwrap().rows_hit_count);
        assert_eq!(
            HashSet::from([8, 0, 6]),
            boards.get(0).unwrap().matched_numbers
        );

        assert_eq!([0, 1, 1, 0, 1], boards.get(1).unwrap().cols_hit_count);
        assert_eq!([1, 0, 1, 0, 1], boards.get(1).unwrap().rows_hit_count);
        assert_eq!(
            HashSet::from([8, 0, 6]),
            boards.get(1).unwrap().matched_numbers
        );
    }

    #[test]
    fn parse_boards2_isok() {
        let input = "

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";
        let boards = parse_boards(input.split("\n\n"), &[7, 4, 9, 5, 11]);
        assert_eq!(3, boards.len(), "want 3 boards, got {}", boards.len());
        assert_eq!([0, 1, 0, 2, 2], boards.get(0).unwrap().cols_hit_count);
        assert_eq!([1, 1, 2, 1, 0], boards.get(0).unwrap().rows_hit_count);
        assert_eq!(
            HashSet::from([7, 4, 9, 5, 11]),
            boards.get(0).unwrap().matched_numbers
        );
    }

    #[test]
    fn part1_isok() {
        let input = "\
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
    8  2 23  4 24
    21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19

    3 15  0  2 22
    9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
    2  0 12  3  7";
        assert_eq!(4512, run1(&input).unwrap())
    }

    #[test]
    fn part2_isok() {
        let input = "\
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
    8  2 23  4 24
    21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19

    3 15  0  2 22
    9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
    2  0 12  3  7";
        assert_eq!(1924, run2(&input).unwrap())
    }
}
