use std::{fs::read_to_string, vec};

fn main() {
    let mut val = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut it = line.split("   ");
            let (Some(str1), Some(str2)) = (it.next(), it.next()) else {
                panic!("Line does not match expectations {}", line)
            };
            (str1.parse::<i64>().unwrap(), str2.parse::<i64>().unwrap())
        })
        .fold(
            (vec![0i64; 0], vec![0i64; 0]),
            |mut vec_tup, (left, right)| {
                vec_tup.0.push(left);
                vec_tup.1.push(right);
                vec_tup
            },
        );
    val.0.sort();
    val.1.sort();
    println!(
        "Result: {:?}",
        val.0
            .iter()
            .zip(val.1.iter())
            .map(|(left, right)| (right - left).abs())
            .sum::<i64>()
    );
}
