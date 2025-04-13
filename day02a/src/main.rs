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

fn main() {
    let binding = read_to_string("./src/input.txt").unwrap();
    let val = binding
        .lines()
        .map(|line| line.split(" ").map(|strnum| strnum.parse::<i32>().unwrap()))
        .map(|row| {
            row.fold(Unknown, |current_direction, current_value| {
                match (current_direction, current_value) {
                    (Unknown, value) => First(value),
                    (First(prev), value) => match get_direction(prev, value) {
                        Same => Inconsistent,
                        direction @ Ascending(_, _) | direction @ Descending(_, _) => {
                            get_dir_if_in_range(direction)
                        }
                    },

                    (Dir(Ascending(prev, _)), value) => match get_direction(prev, value) {
                        direction @ Ascending(_, _) => get_dir_if_in_range(direction),
                        _ => Inconsistent,
                    },
                    (Dir(Descending(prev, _)), value) => match get_direction(prev, value) {
                        direction @ Descending(_, _) => get_dir_if_in_range(direction),
                        _ => Inconsistent,
                    },
                    _ => Inconsistent,
                }
            })
        })
        .filter(|pattern| match pattern {
            Inconsistent => false,
            _ => true,
        });

    println!("{}", val.collect::<Vec<_>>().len())
}
