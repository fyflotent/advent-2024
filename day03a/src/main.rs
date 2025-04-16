use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let binding = read_to_string("./src/input.txt").unwrap();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results: Vec<(i64, i64)> = vec![];
    for (_, [num1, num2]) in re.captures_iter(&binding).map(|c| c.extract()) {
        results.push((num1.parse::<i64>().unwrap(), num2.parse::<i64>().unwrap()));
    }
    println!(
        "{:?}",
        results.into_iter().map(|(n1, n2)| n1 * n2).sum::<i64>()
    );
}
