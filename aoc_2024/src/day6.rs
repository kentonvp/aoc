// Day 6 //////////////////////////////////////////////////////////////////////
use std::collections::HashMap;
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

fn part1(contents: &str) -> usize {
    // Convert the contents in a 2D grid of Nodes
    //     x0, x1, x2, x3, x4, x5]
    // y0 [ 0,  1,  2,  3,  4,  5]
    // y1 [ 0,  1,  2,  3,  4,  5]
    let mut mark: Point = Point { x: -1, y: -1 };
    let mut map = vec![];
    for (y, line) in contents.lines().enumerate() {
        map.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            map[y].push(c == '#');

            if c == '^' {
                mark = Point {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }

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
        if inbounds(
            &map,
            &Point {
                x: next_x,
                y: next_y,
            },
        ) && map[next_y as usize][next_x as usize]
        {
            match current_direction {
                Direction::North => current_direction = Direction::East,
                Direction::East => current_direction = Direction::South,
                Direction::South => current_direction = Direction::West,
                Direction::West => current_direction = Direction::North,
            }
        } else {
            // Take step.
            mark = Point {
                x: next_x,
                y: next_y,
            };
        }
    }

    visited.len()
}

fn part2(contents: &str) -> usize {
    let mut mark: Point = Point { x: -1, y: -1 };
    let mut map = vec![];
    for (y, line) in contents.lines().enumerate() {
        map.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            map[y].push(c == '#');

            if c == '^' {
                mark = Point {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }

    // Marker always starts facing North.
    let mut current_direction = Direction::North;

    let mut visited = HashMap::new();
    let mut cycles = HashSet::new();
    //let mut turned = false;
    while inbounds(&map, &mark) {
        if visited.contains_key(&mark) && !cycles.contains(&(mark, current_direction)) {
            match (visited.get(&mark).unwrap(), current_direction) {
                (Direction::North, Direction::West) => cycles.insert((mark, current_direction)),
                (Direction::East, Direction::North) => cycles.insert((mark, current_direction)),
                (Direction::South, Direction::East) => cycles.insert((mark, current_direction)),
                (Direction::West, Direction::South) => cycles.insert((mark, current_direction)),
                _ => false,
            };
        }
        visited.insert(mark, current_direction);

        // Check next location.
        let (next_x, next_y) = match current_direction {
            Direction::North => (mark.x, mark.y - 1),
            Direction::East => (mark.x + 1, mark.y),
            Direction::South => (mark.x, mark.y + 1),
            Direction::West => (mark.x - 1, mark.y),
        };
        if inbounds(
            &map,
            &Point {
                x: next_x,
                y: next_y,
            },
        ) && map[next_y as usize][next_x as usize]
        {
            match current_direction {
                Direction::North => current_direction = Direction::East,
                Direction::East => current_direction = Direction::South,
                Direction::South => current_direction = Direction::West,
                Direction::West => current_direction = Direction::North,
            }
            //turned = true;
        } else {
            mark = Point {
                x: next_x,
                y: next_y,
            };
            //turned = false;
        }
    }

    cycles.len()
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
