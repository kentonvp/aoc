// Day 6 //////////////////////////////////////////////////////////////////////
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn inbounds(map: &[Vec<bool>], p: &Point) -> bool {
    p.y >= 0 && p.y < map.len() as i32 && p.x >= 0 && p.x < map[0].len() as i32
}

/// Parse the map returning the 2D grid and the guards starting position.
fn parse_map(contents: &str) -> (Vec<Vec<bool>>, Point) {
    let mut map = vec![];
    let mut marker = Point { x: -1, y: -1 };
    for (y, line) in contents.lines().enumerate() {
        map.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            map.last_mut().unwrap().push(c == '#');

            if c == '^' {
                marker = Point { x: x as i32, y: y as i32 };
            }
        }
    }
    (map, marker)
}

fn part1(contents: &str) -> usize {
    // Convert the contents in a 2D grid of Nodes
    //     x0, x1, x2, x3, x4, x5]
    // y0 [ 0,  1,  2,  3,  4,  5]
    // y1 [ 0,  1,  2,  3,  4,  5]
    let (map, mut mark) = parse_map(contents);

    // Marker always starts facing North.
    let mut current_direction = Direction::North;

    let mut visited = HashSet::new();
    while inbounds(&map, &mark) {
        visited.insert(mark);

        // Check next location.
        let (next_x, next_y) = match current_direction {
            Direction::North => (mark.x, mark.y - 1),
            Direction::East => (mark.x + 1, mark.y),
            Direction::South => (mark.x, mark.y + 1),
            Direction::West => (mark.x - 1, mark.y),
        };

        // If next step is inbounds but is an obstacle then change direction.
        if inbounds(&map, &Point { x: next_x, y: next_y })
            && map[next_y as usize][next_x as usize]
        {
            match current_direction {
                Direction::North => current_direction = Direction::East,
                Direction::East => current_direction = Direction::South,
                Direction::South => current_direction = Direction::West,
                Direction::West => current_direction = Direction::North,
            }
        } else {
            // Take step.
            mark = Point { x: next_x, y: next_y };
        }
    }

    visited.len()
}

fn is_cycle(map: &[Vec<bool>], guard: &Point) -> bool {
    let mut seen = HashSet::new();

    let mut guard = *guard;
    let mut current_direction = Direction::North;
    seen.insert((guard, current_direction));

    loop {
        // Step with the guard
        // Check next location.
        let (x, y) = match current_direction {
            Direction::North => (guard.x, guard.y - 1),
            Direction::East => (guard.x + 1, guard.y),
            Direction::South => (guard.x, guard.y + 1),
            Direction::West => (guard.x - 1, guard.y),
        };

        if x < 0 || x >= map[0].len() as i32 || y < 0 || y >= map.len() as i32 {
            return false;
        }

        if map[y as usize][x as usize] {
            current_direction = match current_direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
        } else {
            guard = Point { x, y };
            if seen.contains(&(guard, current_direction)) {
                return true;
            }
            seen.insert((guard, current_direction));
        }
    }
}

fn part2(contents: &str) -> usize {
    let (mut map, guard) = parse_map(contents);

    let mut cycles = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !(map[y][x] || (guard.x as usize == x && guard.y as usize == y))
            {
                // Set map location to true
                map[y][x] = true;
                if is_cycle(&map, &guard) {
                    cycles += 1;
                }
                // Reset after cycle check.
                map[y][x] = false;
            }
        }
    }
    cycles
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
        let contents = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part1(contents), 41)
    }

    #[test]
    fn test_part2() {
        let contents = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part2(contents), 6)
    }
}
#[test]
fn test_is_cycle() {
    let contents = r"....#.....
.........#
..........
..#.#.....
.......#..
..........
.#..^.....
......#.#.
#.........
......#...";
    let (map, guard) = parse_map(contents);
    assert_eq!(is_cycle(&map, &guard), true)
}
