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


fn count_neighbours(grid: &[Vec<char>], r: usize, c: usize) -> u8 {
    let rows = grid.len();
    let cols = grid[0].len();

    DIRS.iter()
        .filter_map(|&(dr, dc)| {
            let nr = r.checked_add_signed(dr)?;
            let nc = c.checked_add_signed(dc)?;
            (nr < rows && nc < cols).then(|| grid[nr][nc])
        })
        .filter(|&cell| cell == '@')
        .count() as u8
}

fn find_accessible_rolls(grid: &[Vec<char>]) ->  Vec<(usize, usize)> {
    let mut accessible = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' && count_neighbours(grid, r, c) < 4 {
                accessible.push((r, c));
            }
        }
    }

    accessible
}


fn part1(input: &str) -> usize {
    let grid = parse_grid(input);
    let mut count = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == '@' && count_neighbours(&grid, r, c) < 4 {
                count += 1;
            }
        }
    }

    count
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
