use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let result = read_to_string("./src/input.txt")
        .map(|result| {
            return Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
                .unwrap()
                .captures_iter(&result)
                .map(|c| c.extract::<2>())
                .map(|(_, res)| {
                    res.iter()
                        .map(|num| num.parse::<i64>().unwrap())
                        .product::<i64>()
                })
                .sum::<i64>();
        })
        .unwrap();
    println!("{}", result);
}
