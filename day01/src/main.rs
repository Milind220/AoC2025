use common::{math::count_zero_crossings, *};

const TOTAL: i32 = 100; 


fn part1(input: &str) -> usize {
    let mut pos = 50u32;
    let mut zeros = 0;
    let instructions = parse_lr_instructions(&input);
    for (dir, num) in instructions {
        let delta = match dir {
            'L' => -(num as i32),
            'R' => num as i32,
            _ => 0,
        };
        pos = mod_wrap_add(pos, delta, TOTAL);
        if pos == 0 {
            zeros += 1;
        } 
    }
    println!("final number: {}", pos);
    zeros
}

fn part2(input: &str) -> usize { 
    let mut pos = 50u32;
    let mut zero_crossings = 0;
    let instructions = parse_lr_instructions(&input);
    for (dir, num) in instructions {
        let delta = match dir {
            'L' => -(num as i32),
            'R' => num as i32,
            _ => 0,
        };
        zero_crossings += count_zero_crossings(pos, delta, TOTAL) as usize;
        pos = mod_wrap_add(pos, delta, TOTAL);
    }
    zero_crossings
}

fn main() {
    let input = std::fs::read_to_string("day01/input.txt").expect("Not found - must've forgotten to put it into the root ya fool");
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
