use regex::Regex;
use std::fs::read_to_string;

fn get_multiples(section: &str) -> Vec<(i64, i64)> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    return re
        .captures_iter(section)
        .map(|c| c.extract())
        .map(|(_, [num1, num2])| (num1.parse::<i64>().unwrap(), num2.parse::<i64>().unwrap()))
        .collect();
}

fn main() {
    let binding = read_to_string("./src/input.txt").unwrap();

    let mut between_val = match binding.find("don't") {
        Some(pos) => Some((0usize, pos)),
        None => None,
    };

    let mut multiples: Vec<(i64, i64)> = vec![];
    while let Some((start, end)) = between_val {
        let curr_string = &binding[start..end];
        multiples.append(&mut get_multiples(&curr_string));

        let rest = &binding[end..];
        let next_do = match rest.find("do()") {
            Some(val) => val,
            _ => break,
        };
        let after_do = &rest[next_do..];
        let next_dont = after_do.find("don't").or(Some(after_do.len()));
        between_val = next_dont.map(|n| (next_do + end, next_do + end + n))
    }

    println!(
        "{:?}",
        multiples.into_iter().map(|(n1, n2)| n1 * n2).sum::<i64>()
    );
}
