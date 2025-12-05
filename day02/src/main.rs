use std::collections::HashSet;

fn find_bounding_ranges(s: &str) -> (u64, u64) {
    let all_nums: Vec<u64> = s
        .trim()
        .split(|c| c == '-' || c == ',')
        .filter_map(|str_num| {
            match str_num.parse::<u64>() {
                Ok(num) => {
                    println!("Successfully parsed: {}", num);
                    Some(num)
                }
                Err(e) => {
                    println!("Error parsing '{}': {}", str_num, e);
                    None
                }
            }
        })
        .collect();
    let min = all_nums.iter().min().copied().unwrap();
    let max = all_nums.iter().max().copied().unwrap();
    (min, max)
}

fn generate_all_mirrored() -> Vec<u64> {
    let mut mirrored = Vec::new();

    // k = number of digits in the half
    for k in 1..=5 {
        let pow_k = 10u64.pow(k); // 10^k
        let start = 10u64.pow(k - 1);       

        for half in start..pow_k {
            let full = half * pow_k + half;
            mirrored.push(full);
        }
    }

    mirrored
}

fn generate_all_bad_patterns() -> Vec<u64> {
    let mut naughty: HashSet<u64> = HashSet::new();
    let max_digits = 10;

    for k in 1..=max_digits / 2 {  // k=1 to 5
        // k = digits in one block
        let start = 10u64.pow((k - 1) as u32);  // e.g. k=3 → 100
        let end = 10u64.pow(k as u32);         // e.g. k=3 → 1000

        // Possible total lengths: must be multiple of k, and at least 2k
        for &total_len in &(k * 2..=max_digits)
            .step_by(k)
            .collect::<Vec<_>>()
        {
            let blocks = total_len / k;  // how many times the pattern repeats

            for base in start..end {
                let mut num = 0u64;
                for i in 0..blocks {
                    num += base * 10u64.pow((i * k) as u32);
                }
                naughty.insert(num);
            }
        }
    }

    let bad: Vec<u64> = naughty.into_iter().collect();
    bad
}

fn parse_ranges(s: &str) -> Vec<(u64, u64)> {
    let all_nums: Vec<u64> = s
        .trim()
        .split(|c| c == '-' || c == ',')
        .filter_map(|str_num| {
            match str_num.parse::<u64>() {
                Ok(num) => {
                    Some(num)
                }
                Err(e) => {
                    None
                }
            }
        })
        .collect();

    let ranges: Vec<(u64, u64)> = all_nums
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some((chunk[0], chunk[1]))
            } else {
                None
            }
        })
        .collect();

    ranges
}



fn part1(input: &str) -> usize { 
    let all_mirrored = generate_all_mirrored();
    println!("Total mirrored numbers: {}", all_mirrored.len());

    let ranges = parse_ranges(&input);

    let answer: u64 = all_mirrored
        .iter()
        .copied()
        .filter(|&n| ranges.iter().any(|&(lo, hi)| lo <= n && n <= hi))
        .sum();

    answer as usize 
}

fn part2(input: &str) -> usize { 
    let all_bad = generate_all_bad_patterns();

    let ranges = parse_ranges(&input);
    
    let answer: u64 = all_bad
        .iter()
        .copied()
        .filter(|&n| ranges.iter().any(|&(lo, hi)| lo <= n && n <= hi))
        .sum();

    answer as usize
}

fn main() {
    let input = std::fs::read_to_string("day02/input.txt").unwrap();
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
