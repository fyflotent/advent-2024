use Direction::*;
use Pattern::*;
use std::fs::read_to_string;

enum Direction {
    Ascending(i32, i32),
    Descending(i32, i32),
    Same,
}

enum Pattern {
    Unknown,
    First(i32),
    Dir(Direction),
    Retry(Vec<i32>, Option<Vec<i32>>),
    Inconsistent,
}

fn get_direction(prev: i32, value: i32) -> Direction {
    let diff = prev - value;
    if diff > 0 {
        Descending(value, diff.abs())
    } else if diff < 0 {
        Ascending(value, diff.abs())
    } else {
        Same
    }
}

fn get_dir_if_in_range(direction: Direction) -> Pattern {
    let diff = match direction {
        Ascending(_, diff) | Descending(_, diff) => diff,
        _ => panic!("Incorrect value passed into get_dir_if_in_range"),
    };
    if diff >= 1 && diff <= 3 {
        Dir(direction)
    } else {
        Inconsistent
    }
}

fn check_row(row: Vec<i32>) -> Pattern {
    println!("{:?}", row);
    return row.iter().enumerate().fold(
        Unknown,
        |current_direction, (index, current_value)| match (current_direction, current_value) {
            (Unknown, value) => First(*value),
            (First(prev), value) => match get_direction(prev, *value) {
                Same => {
                    let mut row_copy = row.clone();
                    row_copy.remove(index);
                    let mut row_copy_2 = row.clone();
                    row_copy_2.remove(index - 1);
                    Retry(row_copy, Some(row_copy_2))
                }
                direction @ Ascending(_, _) | direction @ Descending(_, _) => {
                    get_dir_if_in_range(direction)
                }
            },

            (Dir(Ascending(prev, _)), value) => match get_direction(prev, *value) {
                direction @ Ascending(_, _) => get_dir_if_in_range(direction),
                _ => {
                    let mut row_copy = row.clone();
                    row_copy.remove(index);
                    let mut row_copy_2 = row.clone();
                    row_copy_2.remove(index - 1);
                    Retry(row_copy, Some(row_copy_2))
                }
            },
            (Dir(Descending(prev, _)), value) => match get_direction(prev, *value) {
                direction @ Descending(_, _) => get_dir_if_in_range(direction),
                _ => {
                    let mut row_copy = row.clone();
                    row_copy.remove(index);
                    let mut row_copy_2 = row.clone();
                    row_copy_2.remove(index - 1);
                    Retry(row_copy, Some(row_copy_2))
                }
            },
            a => a.0,
        },
    );
}

fn main() {
    let binding = read_to_string("./src/input.txt").unwrap();
    let val = binding
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|strnum| strnum.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|row| match check_row(row) {
            Retry(retry_row, retry_row_2_option) => match check_row(retry_row) {
                Retry(_, _) => match retry_row_2_option {
                    Some(retry_row_2) => check_row(retry_row_2),
                    _ => Inconsistent,
                },

                a => a,
            },
            pattern => pattern,
        })
        .filter(|pattern| match pattern {
            Inconsistent => false,
            Retry(_, _) => false,
            _ => true,
        });

    println!("{}", val.collect::<Vec<_>>().len())
}
