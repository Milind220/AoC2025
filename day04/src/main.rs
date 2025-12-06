fn parse_grid(s: &str) -> Vec<Vec<char>> {
    let mut parsed_grid = Vec::new();
    for line in s.lines() {
        let parsed_row = line
            .trim()
            .chars()
            .collect();
        parsed_grid.push(parsed_row);
        }

    parsed_grid
}

const DIRS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),         (0, 1),
    (1, -1), (1, 0), (1, 1)
];


fn count_neighbours(grid: Vec<Vec<char>>, r: usize, c: usize) -> u8 {
    let rows = grid.len();
    let cols = grid[0].len();

    let neighbours = DIRS.iter()
        .filter_map(|&(dr, dc)| {
            let nr = r.checked_add_signed(dr)?;
            let nc = c.checked_add_signed(dc)?;
            (nr < rows && nc < cols).then(|| grid[nr][nc])
        })
        .filter(|&cell| cell == '@')
        .count();

    neighbours as u8
}


fn part1(input: &str) -> usize {
    let grid = parse_grid(&input);

    let mut forkliftable_rolls = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            let neighbours = count_neighbours(grid.clone(), r, c);
            if neighbours < 4 && grid[r][c] == '@'{
                forkliftable_rolls += 1
            }
        }
    }

    forkliftable_rolls
}
/*
fn part2(input: &str) -> usize { todo! () }
*/

fn main() {
    let input = std::fs::read_to_string("day04/input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    /*
    println!("Part 2: {}", part2(&input));
*/
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../day04/test.txt");
    #[test] fn test_part1() { assert_eq!(part1(INPUT), 13); }
    /*
    #[test] fn test_part2() { assert_eq!(part2(INPUT), 0); }
    */
}
