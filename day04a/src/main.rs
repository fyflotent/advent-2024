fn get_diagonal_count(lines: &Vec<Vec<char>>) -> i64 {
    let mut diagonal_count = 0;
    for i in 0..lines.len() {
        let mut current_4: Vec<char> = vec![];
        for j in (0..=i).rev() {
            let new_char = lines[i - j][j];

            if current_4.len() >= 3 {
                if current_4.len() == 4 {
                    current_4 = current_4[1..].to_vec();
                }

                current_4.push(new_char);
                let curr_str = String::from_iter(current_4.iter());
                if curr_str == "XMAS" || curr_str == "SAMX" {
                    diagonal_count += 1;
                }
            } else {
                current_4.push(new_char);
            }
        }
    }
    for i in 0..lines.len() - 1 {
        let mut current_4: Vec<char> = vec![];
        for j in (0..=i).rev() {
            let new_char = lines[(lines.len() - 1) - j][(lines.len() - 1) - (i - j)];

            if current_4.len() >= 3 {
                if current_4.len() == 4 {
                    current_4 = current_4[1..].to_vec();
                }

                current_4.push(new_char);
                let curr_str = String::from_iter(current_4.iter());
                if curr_str == "XMAS" || curr_str == "SAMX" {
                    diagonal_count += 1;
                }
            } else {
                current_4.push(new_char);
            }
        }
    }
    diagonal_count
}

fn main() {
    let input = include_str!("./input.txt");
    let all_lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rev_lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .rev()
        .collect();
    let right_diagonal_count = get_diagonal_count(&all_lines);
    let left_diagonal_count = get_diagonal_count(&rev_lines);

    let vertical_input = (0..all_lines.len())
        .map(|index| all_lines.iter().map(move |line| line[index]))
        .map(String::from_iter)
        .collect::<Vec<String>>()
        .join("\n");

    println!(
        "{}",
        right_diagonal_count
            + left_diagonal_count
            + input.match_indices("XMAS").count() as i64
            + input.match_indices("SAMX").count() as i64
            + vertical_input.match_indices("XMAS").count() as i64
            + vertical_input.match_indices("SAMX").count() as i64
    );
}

/*
6  10 13 15
3  7  11 14
1  4  8  12
0  2  5  9
      i j
3,3 = 0 0| 3-i, 3-i
3,2 = 1 0| 3-(j), 3-1
2,3 = 1 1| 3-(j), 3 -0
3,1 = 2 0| j, 3 - (i-j)
2,2 = 2 1| j, 3 - 1
1,3 = 2 2| j, 3 - 0

*/
