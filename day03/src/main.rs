fn parse_batt_banks(s: &str) -> Vec<&[u8]> {
    let mut batt_banks: Vec<&[u8]> = Vec::new();

    for line in s.lines() {
        batt_banks.push(line.as_bytes());
    }
    batt_banks
}

fn get_highest_2_battery_joltage(bank: &[u8], bank_len: usize) -> u32 {
    let max_digit = bank.iter().max().unwrap();
    let leftmost_pos = bank.iter().position(|&b| b==*max_digit).unwrap();

    if leftmost_pos==bank.len()-1 {
        let left_digit = bank[..bank_len-1].iter().max().unwrap();
        return (((left_digit - b'0') * 10u8) + (max_digit - b'0')) as u32;
    } else {
        let right_max = bank[leftmost_pos+1..].iter().max().unwrap();
        return (((max_digit - b'0') * 10u8) + (right_max - b'0')) as u32;
    }
}

fn part1(input: &str) -> usize {
    let battery_banks = parse_batt_banks(&input);
    let bank_len = battery_banks[0].len(); // Since they are all the same length this is fine.
    let mut joltage = 0u32;

    for bank in battery_banks {
        joltage += get_highest_2_battery_joltage(bank, bank_len);
    }

    joltage as usize
}

fn part2(input: &str) -> usize {
    42
}

fn main() {
    let input = std::fs::read_to_string("day03/input.txt").unwrap();
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
