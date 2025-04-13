use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let binding = read_to_string("./src/input.txt").unwrap();
    let (arr, hash) = binding
        .lines()
        .map(|line| match line.split("   ").collect::<Vec<&str>>()[..] {
            [s1, s2, ..] => (s1.parse::<i64>().unwrap(), s2.parse::<i64>().unwrap()),
            _ => panic!("Line does not match expectations {}", line),
        })
        .fold(
            (vec![], HashMap::<i64, i64>::new()),
            |(mut list, mut hash), (left, right)| {
                list.push(left);

                match hash.get(&right) {
                    Some(current_count) => hash.insert(right, *current_count + 1),
                    _ => hash.insert(right, 1),
                };
                (list, hash)
            },
        );

    println!(
        "{}",
        arr.iter()
            .map(|curr| match hash.get(curr) {
                Some(count) => count * curr,
                _ => 0,
            })
            .sum::<i64>()
    );
}
