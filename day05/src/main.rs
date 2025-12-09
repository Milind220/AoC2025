use std::collections::HashSet;

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut lines = input.lines();

    let mut fresh_ranges = Vec::new();
    for line in lines.by_ref() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        if let Some((start, end)) = line.split_once('-') {
            let start = start.trim().parse::<u64>().expect("valid start");
            let end = end.trim().parse::<u64>().expect("valid end");
            fresh_ranges.push((start, end));
        }
    }

    let mut available = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let id = line.parse::<u64>().expect("valid ingredient ID");
        available.push(id);
    }

    (fresh_ranges, available)
}

fn part1(input: &str) -> u64 {
    let (fresh_ranges, available) = parse_input(input);

    let mut count = 0;
    for &ingredient in &available {
        if fresh_ranges.iter().any(|&(lo, hi)| ingredient >= lo && ingredient <= hi) {
            count += 1;
        }
    }

    count
}

fn part1_hashset(input: &str) -> u64 {
    let (fresh_ranges, available) = parse_input(input);

    let mut fresh_ids: HashSet<u64> = HashSet::new();
    for &(lo, hi) in &fresh_ranges {
        fresh_ids.extend(lo..=hi);
    }

    available.iter().filter(|&&id| fresh_ids.contains(&id)).count() as u64
}

/*
fn part2(input: &str) -> usize {
    todo!()
}
*/

fn main() {
    let input = std::fs::read_to_string("day05/input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    /*
    println!("Part 2: {}", part2(&input));
*/
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 0);
    }
    /*
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 0);
    }
    */
}
