type Grid = Vec<Vec<u8>>; // 1 = roll exists, 0 = empty.

fn parse_grid(s: &str) -> Vec<Vec<char>> {
    let mut parsed_grid = Vec::new();
    for line in s.lines() {
        let parsed_row = line.trim().chars().collect();
        parsed_grid.push(parsed_row);
    }

    parsed_grid
}

fn to_bitmap(grid: &[Vec<char>]) -> Grid {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|&cell| if cell == '@' { 1 } else { 0 })
                .collect()
        })
        .collect()
}

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
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

fn count_neighbors_bit(grid: &[Vec<u8>], r: usize, c: usize) -> u8 {
    DIRS.iter()
        .filter_map(|&(dr, dc)| {
            let nr = r.checked_add_signed(dr)?;
            let nc = c.checked_add_signed(dc)?;
            (nr < grid.len() && nc < grid[0].len()).then(|| grid[nr][nc])
        })
        .filter(|&cell| cell == 1)
        .count() as u8
}

fn part2(input: &str) -> usize {
    let original_grid = parse_grid(input);
    let mut grid = to_bitmap(&original_grid);
    let mut total = 0;

    loop {
        let mut to_remove = Vec::new();
        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if cell == 1 && count_neighbors_bit(&grid, r, c) < 4 {
                    total += 1;
                    to_remove.push((r, c));
                }
            }
        }
        if to_remove.is_empty() {
            break;
        } else {
            for (r, c) in to_remove {
                grid[r][c] = 0;
            }
        }
    }

    total
}

fn main() {
    let input = std::fs::read_to_string("day04/input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../day04/test.txt");
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 13);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 42);
    }
}
