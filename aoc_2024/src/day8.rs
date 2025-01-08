// Day 8 //////////////////////////////////////////////////////////////////////
use std::collections::HashSet;

struct Grid {
    width: i32,
    height: i32,
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn is_grid(&self, p: (i32, i32)) -> bool {
        0 <= p.0 && p.0 < self.width && 0 <= p.1 && p.1 < self.height
    }

    fn find_pairs(
        &self,
        freq: &char,
        p: (i32, i32),
    ) -> HashSet<((i32, i32), (i32, i32))> {
        let mut antennas = HashSet::new();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, f) in row.iter().enumerate() {
                // Convert to i32 for comparison...
                let x = x as i32;
                let y = y as i32;

                if freq == f && (x, y) != p {
                    if y < p.1 || (y == p.1 && x > p.0) {
                        antennas.insert(((x, y), p));
                    } else {
                        antennas.insert((p, (x, y)));
                    }
                }
            }
        }
        antennas
    }
}

fn parse_grid(contents: &str) -> Grid {
    let grid = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    Grid { width: grid[0].len() as i32, height: grid.len() as i32, grid }
}

fn apply_delta(
    p: (i32, i32),
    delta: (i32, i32),
    neg: bool,
) -> Option<(i32, i32)> {
    let (x, y) = match neg {
        true => (p.0.checked_sub(delta.0), p.1.checked_sub(delta.1)),
        false => (p.0.checked_add(delta.0), p.1.checked_add(delta.1)),
    };

    match (x, y) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}
fn part1(contents: &str) -> u32 {
    let grid = parse_grid(contents);

    let mut antinodes = HashSet::new();

    for (y, row) in grid.grid.iter().enumerate() {
        for (x, freq) in row.iter().enumerate() {
            let x = x as i32;
            let y = y as i32;
            if !freq.is_alphanumeric() {
                continue;
            }
            let pairs = &grid.find_pairs(freq, (x, y));
            for (p1, p2) in pairs.iter() {
                let delta_x = p2.0 - p1.0;
                let delta_y = p2.1 - p1.1;
                if let Some(p0) = apply_delta(*p1, (delta_x, delta_y), true) {
                    if grid.is_grid(p0) {
                        antinodes.insert(p0);
                    }
                }
                if let Some(p0) = apply_delta(*p2, (delta_x, delta_y), false) {
                    if grid.is_grid(p0) {
                        antinodes.insert(p0);
                    }
                }
            }
        }
    }

    antinodes.len() as u32
}

//fn print_grid(grid: &Grid, p: (i32, i32)) {
//    for (y, row) in grid.grid.iter().enumerate() {
//        for (x, f) in row.iter().enumerate() {
//            let x = x as i32;
//            let y = y as i32;
//            if (x, y) == p {
//                print!("#");
//            } else {
//                print!("{}", f);
//            }
//        }
//        println!();
//    }
//}

fn part2(contents: &str) -> u32 {
    let grid = parse_grid(contents);

    let mut antinodes = HashSet::new();
    let mut seen = HashSet::new();

    for (y, row) in grid.grid.iter().enumerate() {
        for (x, freq) in row.iter().enumerate() {
            let x = x as i32;
            let y = y as i32;
            if !freq.is_alphanumeric() {
                continue;
            }
            let pairs = &grid.find_pairs(freq, (x, y));
            for (p1, p2) in pairs.iter() {
                let delta_x = p2.0 - p1.0;
                let delta_y = p2.1 - p1.1;

                let mut p1 = (p1.0, p1.1);
                let mut p2 = (p2.0, p2.1);

                if seen.contains(&(p1, p2)) {
                    continue;
                }
                seen.insert((p1, p2));

                // Add the antennas as anitnodes!!
                antinodes.insert(p1);
                antinodes.insert(p2);

                let mut progress = true;
                while progress {
                    progress = false;
                    if let Some(p0) = apply_delta(p1, (delta_x, delta_y), true)
                    {
                        if grid.is_grid(p0) {
                            antinodes.insert(p0);
                            p1 = (p0.0, p0.1);
                            progress = true;
                        }
                    }

                    if let Some(p0) = apply_delta(p2, (delta_x, delta_y), false)
                    {
                        if grid.is_grid(p0) {
                            antinodes.insert(p0);
                            p2 = (p0.0, p0.1);
                            progress = true;
                        }
                    }
                }
            }
        }
    }

    antinodes.len() as u32
}

pub fn solve(contents: &str) {
    let time = std::time::Instant::now();
    let res = part1(contents);
    println!("Part1: {} ({:.4}s)", res, time.elapsed().as_secs_f64());

    let time = std::time::Instant::now();
    let res = part2(contents);
    println!("Part2: {} ({:.4}s)", res, time.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part1(contents), 14)
    }

    #[test]
    fn test_part2() {
        let contents = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part2(contents), 34)
    }
}
