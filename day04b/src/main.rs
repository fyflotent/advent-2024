fn main() {
    let all_lines: Vec<Vec<char>> = include_str!("./input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut count = 0;
    for i in 0..all_lines.len() - 2 {
        for j in 0..all_lines.len() - 2 {
            let left_diagonal = String::from_iter([
                all_lines[i][j],
                all_lines[i + 1][j + 1],
                all_lines[i + 2][j + 2],
            ]);
            let right_diagonal = String::from_iter([
                all_lines[i + 2][j],
                all_lines[i + 1][j + 1],
                all_lines[i][j + 2],
            ]);
            if (left_diagonal == "MAS" || left_diagonal == "SAM")
                && (right_diagonal == "MAS" || right_diagonal == "SAM")
            {
                count += 1;
            }
        }
    }

    println!("{}", count)
}
