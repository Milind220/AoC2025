use std::iter::zip;

fn parse_input(input: &str) -> (Vec<Vec<u16>>, Vec<char>) {
    let lines: Vec<&str> = input.lines().collect();
    let num_rows = lines.len();

    let mut rows: Vec<Vec<u16>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if i < (num_rows - 1) {
            // Then it's a number row
            rows.push(
                line.split_whitespace()
                    .filter_map(|str_num| str_num.parse::<u16>().ok())
                    .collect(),
            );
        } else {
            // then it's the operations row
            operations = line
                .trim()
                .chars()
                .filter(|&c| c == '*' || c == '+')
                .collect();
        }
    }
    (rows, operations)
}

fn part1(input: &str) -> u64 {
    let (rows, operations) = parse_input(input);

    // transpose it to have columns instead of rows
    let columns: Vec<Vec<u64>> = (0..rows[0].len())
        .map(|col| rows.iter().map(|row| row[col] as u64).collect())
        .collect();

    zip(columns, operations)
        .map(|(col, op)| -> u64 {
            match op {
                '+' => col.iter().sum(),
                '*' => col.iter().product(),
                _ => panic!("WTF"),
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    todo!()
}

fn main() {
    let input = std::fs::read_to_string("day06/input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 0);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
}
