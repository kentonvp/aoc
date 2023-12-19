use std::collections::HashSet;

fn is_north_sink(c: char) -> bool {
    matches!(c, '7' | 'F' | '|' | 'S')
}

fn is_west_sink(c: char) -> bool {
    matches!(c, 'L' | 'F' | '-' | 'S')
}

fn is_east_sink(c: char) -> bool {
    matches!(c, 'J' | '7' | '-' | 'S')
}

fn is_south_sink(c: char) -> bool {
    matches!(c, 'L' | 'J' | '|' | 'S')
}

fn find_start(maze: &[Vec<char>]) -> (usize, usize) {
    for (y, line) in maze.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                return (x, y);
            }
        }
    }
    panic!("No start found");
}

fn find_next(maze: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut next = Vec::new();
    let c = maze[pos.1][pos.0];
    if is_south_sink(c) && pos.1 > 0 && is_north_sink(maze[pos.1 - 1][pos.0]) {
        next.push((pos.0, pos.1 - 1));
    }

    if is_north_sink(c) && pos.1 < maze.len() - 1 && is_south_sink(maze[pos.1 + 1][pos.0]) {
        next.push((pos.0, pos.1 + 1));
    }

    if is_east_sink(c) && pos.0 > 0 && is_west_sink(maze[pos.1][pos.0 - 1]) {
        next.push((pos.0 - 1, pos.1));
    }

    if is_west_sink(c) && pos.0 < maze[0].len() - 1 && is_east_sink(maze[pos.1][pos.0 + 1]) {
        next.push((pos.0 + 1, pos.1));
    }

    next
}

fn get_path(maze: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    path.push(find_start(maze));

    let mut seen: HashSet<(usize, usize)> = HashSet::from_iter(path.iter().cloned());
    'outer: while let Some(curr) = path.pop() {
        for n in find_next(maze, curr) {
            if path.len() > 1 && n == path[0] {
                path.push(curr);
                break 'outer;
            }
            if !seen.contains(&n) {
                path.push(curr);
                path.push(n);
                seen.insert(n);
                continue 'outer;
            }
        }
    }
    path
}

pub fn process1(input: &str) -> u32 {
    let stime = std::time::Instant::now();

    let maze: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let path = get_path(&maze);

    let sol = (path.len() / 2) as u32;
    println!("Day 10 - Part 1: {} [{:?}]", sol, stime.elapsed());
    sol
}

pub fn process2(input: &str) -> u32 {
    let stime = std::time::Instant::now();
    let maze: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = maze.len();
    let width = maze[0].len();
    let path: HashSet<(usize, usize)> = HashSet::from_iter(get_path(&maze));

    let mut sol = 0;
    for (y, _c) in maze.iter().enumerate().take(height - 1).skip(1) {
        for x in 1..(width - 1) {
            if path.contains(&(x, y)) {
                continue;
            }
            let mut cross: usize = 0;
            let mut touching_bottom = false;
            let mut touching_top = false;
            for xi in 0..x {
                if path.contains(&(xi, y)) && "|LF7JS".contains(maze[y][xi]) {
                    if maze[y][xi] == '|' {
                        cross += 1;
                    } else if touching_bottom && maze[y][xi] == '7' {
                        cross += 1;
                        touching_bottom = false;
                    } else if touching_top && maze[y][xi] == 'J' {
                        cross += 1;
                        touching_top = false;
                    } else if !touching_bottom && (maze[y][xi] == 'L' || maze[y][xi] == 'S') {
                        // Kinda hacky, 'S' acts as an 'L' in the input... Way around would be
                        // to replace 'S' with the value it actually represents. then use this
                        // algorithm.
                        touching_bottom = true;
                    } else if !touching_top && maze[y][xi] == 'F' {
                        touching_top = true;
                    } else {
                        touching_top = false;
                        touching_bottom = false;
                    }
                }
            }

            // If odd, then in polygon
            if cross % 2 == 1 {
                sol += 1;
            }
        }
    }

    println!("Day 10 - Part 2: {} [{:?}]", sol, stime.elapsed());
    sol
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1a() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(process1(input), 4)
    }

    #[test]
    fn test_process1b() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(process1(input), 8)
    }

    #[test]
    fn test_process2a() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(process2(input), 4)
    }

    #[test]
    fn test_process2b() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(process2(input), 8)
    }

    #[test]
    fn test_process2c() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(process2(input), 10)
    }
}
