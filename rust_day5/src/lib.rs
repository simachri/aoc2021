use std::collections::HashSet;
use std::error::Error;
use std::iter::zip;

struct Diagram {
    field: Vec<Vec<usize>>,
    points_ge_two: HashSet<(usize, usize)>,
}

pub fn run1(input: &str, include_45degree_lines: bool) -> Result<usize, Box<dyn Error>> {
    let diagram = input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            let (from_x, from_y) = from.split_once(",").unwrap();
            let (to_x, to_y) = to.split_once(",").unwrap();
            (
                (
                    from_x.parse::<usize>().unwrap(),
                    from_y.parse::<usize>().unwrap(),
                ),
                (
                    to_x.parse::<usize>().unwrap(),
                    to_y.parse::<usize>().unwrap(),
                ),
            )
        })
        .fold(
            Diagram {
                field: vec![vec![0; 1000]; 1000],
                points_ge_two: HashSet::new(),
            },
            |mut diagram, vector| {
                let (from, to) = vector;
                println!("Vector: {:?}", (from, to));

                let mut is_diagonal_vec = false;

                if (from.0 == to.0) || (from.1 == to.1) {
                    // Horizontal line. Always included.
                } else {
                    // Diagonal line.
                    if !include_45degree_lines {
                        return diagram;
                    } else {
                        // Check if it is a 45 degree line.
                        if from.0.abs_diff(to.0) == from.1.abs_diff(to.1) {
                            is_diagonal_vec = true;
                        } else {
                            return diagram;
                        }
                    }
                }

                let (x_from, x_to) = match from.0 <= to.0 {
                    true => (from.0, to.0),
                    false => (to.0, from.0),
                };
                let (y_from, y_to) = match from.1 <= to.1 {
                    true => (from.1, to.1),
                    false => (to.1, from.1),
                };

                fn get_range(from: usize, to: usize) -> impl Iterator<Item = usize> {
                    if from <= to {
                        Box::new(from..=to)
                    } else {
                        Box::new((to..=from).rev()) as Box<dyn Iterator<Item = usize>>
                    }
                }

                if is_diagonal_vec {
                    let range_x = get_range(from.0, to.0);
                    let range_y = get_range(from.1, to.1);

                    for (x, y) in zip(range_x, range_y) {
                        diagram.field[x][y] += 1;
                        if diagram.field[x][y] >= 2 {
                            diagram.points_ge_two.insert((x, y));
                        }
                    }
                } else {
                    for x in x_from..=x_to {
                        for y in y_from..=y_to {
                            diagram.field[x][y] += 1;
                            if diagram.field[x][y] >= 2 {
                                diagram.points_ge_two.insert((x, y));
                            }
                        }
                    }
                }
                diagram
            },
        );

    Ok(diagram.points_ge_two.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_isok() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(5, run1(&input, false).unwrap())
    }

    #[test]
    fn part2_isok() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(12, run1(&input, true).unwrap())
    }
}
