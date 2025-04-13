use std::{collections::BinaryHeap, fs::read_to_string};

fn main() {
    let (heap1, heap2) = read_to_string("./src/input.txt")
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
            (BinaryHeap::<i64>::new(), BinaryHeap::<i64>::new()),
            |(mut heap1, mut heap2), (left, right)| {
                heap1.push(left);
                heap2.push(right);
                (heap1, heap2)
            },
        );

    println!(
        "Result: {:?}",
        heap1
            .into_sorted_vec()
            .iter()
            .zip(heap2.into_sorted_vec().iter())
            .map(|(left, right)| (right - left).abs())
            .sum::<i64>()
    );
}
