use std::str::FromStr;

pub fn parse_lr_instructions(s: &str) -> Vec<(char, u32)> {
    s.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let direction = line.chars().next().expect("Empty line after filter!");
            let num = u32::from_str(&line[1..]).expect("Invalid number!");
            (direction, num)
        })
        .collect()
}
