fn part1(input: &str) -> usize { todo! () }
fn part2(input: &str) -> usize { todo! () }

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../test.txt");
    #[test] fn test_part1() { assert_eq!(part1(INPUT), 0); }
    #[test] fn test_part2() { assert_eq!(part2(INPUT), 0); }
}
