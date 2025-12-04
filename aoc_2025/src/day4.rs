const GRID_SIZE: usize = 137;

pub fn part1(input: &str) -> usize {
    let mut total = 0;

    let mut grid = [['.'; GRID_SIZE]; GRID_SIZE];
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[y][x] = ch;
        }
    }

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if grid[y][x] == '@' {
                let mut neighbors = 0;
                if y > 0 && grid[y - 1][x] == '@' {
                    neighbors += 1;
                }
                if y < GRID_SIZE - 1 && grid[y + 1][x] == '@' {
                    neighbors += 1;
                }
                if x > 0 && grid[y][x - 1] == '@' {
                    neighbors += 1;
                }
                if x < GRID_SIZE - 1 && grid[y][x + 1] == '@' {
                    neighbors += 1;
                }
                if y > 0 && x > 0 && grid[y - 1][x - 1] == '@' {
                    neighbors += 1;
                }
                if y > 0 && x < GRID_SIZE - 1 && grid[y - 1][x + 1] == '@' {
                    neighbors += 1;
                }
                if y < GRID_SIZE - 1 && x > 0 && grid[y + 1][x - 1] == '@' {
                    neighbors += 1;
                }
                if y < GRID_SIZE - 1 && x < GRID_SIZE - 1 && grid[y + 1][x + 1] == '@' {
                    neighbors += 1;
                }

                if neighbors < 4 {
                    total += 1;
                }
            }
        }
    }

    println!("Day 4, Part 1: {}", total);
    total
}

pub fn part2(input: &str) -> usize {
    let mut total = 0;

    let mut grid = [['.'; GRID_SIZE]; GRID_SIZE];
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid[y][x] = ch;
        }
    }

    loop {
        let prev_total = total;
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if grid[y][x] == '@' {
                    let mut neighbors = 0;
                    if y > 0 && grid[y - 1][x] == '@' {
                        neighbors += 1;
                    }
                    if y < GRID_SIZE - 1 && grid[y + 1][x] == '@' {
                        neighbors += 1;
                    }
                    if x > 0 && grid[y][x - 1] == '@' {
                        neighbors += 1;
                    }
                    if x < GRID_SIZE - 1 && grid[y][x + 1] == '@' {
                        neighbors += 1;
                    }
                    if y > 0 && x > 0 && grid[y - 1][x - 1] == '@' {
                        neighbors += 1;
                    }
                    if y > 0 && x < GRID_SIZE - 1 && grid[y - 1][x + 1] == '@' {
                        neighbors += 1;
                    }
                    if y < GRID_SIZE - 1 && x > 0 && grid[y + 1][x - 1] == '@' {
                        neighbors += 1;
                    }
                    if y < GRID_SIZE - 1 && x < GRID_SIZE - 1 && grid[y + 1][x + 1] == '@' {
                        neighbors += 1;
                    }

                    if neighbors < 4 {
                        total += 1;
                        grid[y][x] = '.';
                    }
                }
            }
        }
        if prev_total == total {
            break;
        }
    }

    println!("Day 4, Part 2: {}", total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
        assert_eq!(part2(input), 43);
    }
}
