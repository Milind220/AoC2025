fn parse_batt_banks(s: &str) -> Vec<&[u8]> {
    let mut batt_banks: Vec<&[u8]> = Vec::new();

    for line in s.lines() {
        batt_banks.push(line.as_bytes());
    }
    batt_banks
}

fn get_highest_2_battery_joltage(bank: &[u8]) -> u64 {
    let bank_len = bank.len();
    let max_digit = bank.iter().max().unwrap();
    let leftmost_pos = bank.iter().position(|&b| b==*max_digit).unwrap();

    if leftmost_pos==bank.len()-1 {
        let left_digit = bank[..bank_len-1].iter().max().unwrap();
        return (((left_digit - b'0') * 10u8) + (max_digit - b'0')) as u64;
    } else {
        let right_max = bank[leftmost_pos+1..].iter().max().unwrap();
        return (((max_digit - b'0') * 10u8) + (right_max - b'0')) as u64;
    }
}

fn get_highest_n_battery_joltage(bank: &[u8], n: usize) -> u64{
    let bank_len = bank.len();
    let mut highest_n_battery_joltage = 0u64;
    let mut pos = 0usize;
    
    for i in 0..n {
        // We need to leave enough digits after this one to be able to still pick n digits total.
        // We need (n - i - 1) digits after this one.
        // Then max lookup index:
        let last_possible = bank_len - (n - i);
        let slice = &bank[pos..=last_possible];

        let best_digit = slice.iter().max().unwrap();
        let best_digit_index_in_slice = slice.iter().position(|&b| b==*best_digit).unwrap();
        let best_digit_index = pos + best_digit_index_in_slice;

        highest_n_battery_joltage = highest_n_battery_joltage * 10 + (best_digit - b'0') as u64;
        pos = best_digit_index + 1;
    }
    highest_n_battery_joltage
}

fn part1(input: &str) -> usize {
    let battery_banks = parse_batt_banks(&input);
    let mut joltage = 0u64;

    for bank in battery_banks {
        // joltage += get_highest_2_battery_joltage(bank);
        joltage += get_highest_n_battery_joltage(bank, 2);
    }

    joltage as usize
}

fn part2(input: &str) -> usize {
    let battery_banks = parse_batt_banks(&input);
    let mut joltage = 0u64;

    for bank in battery_banks {
        joltage += get_highest_n_battery_joltage(bank, 12);
    }

    joltage as usize
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
