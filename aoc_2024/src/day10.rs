// Day 10 /////////////////////////////////////////////////////////////////////
use std::collections::HashSet;

/// Parse the input into a vector of vectors.
fn parse_input(contents: &str) -> Vec<Vec<u32>> {
    contents
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn print_map(map: &Vec<Vec<u32>>) {
    for row in map {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn count_trails(map: &[Vec<u32>], start: (usize, usize)) -> u32 {
    let height = map.len();
    let width = map[0].len();

    let mut seen = HashSet::new();
    let mut trails = vec![(0u32, start)];
    while let Some(t) = trails.pop() {
        let x = t.1 .0;
        let y = t.1 .1;
        let h = t.0;

        if t.0 == 9 {
            seen.insert(t.1);
            continue;
        }

        if x > 0 && map[y][x - 1] == h + 1 {
            trails.push((h + 1, (x - 1, y)))
        }
        if x < width - 1 && map[y][x + 1] == h + 1 {
            trails.push((h + 1, (x + 1, y)))
        }
        if y < height - 1 && map[y + 1][x] == h + 1 {
            trails.push((h + 1, (x, y + 1)))
        }
        if y > 0 && map[y - 1][x] == h + 1 {
            trails.push((h + 1, (x, y - 1)))
        }
    }

    seen.len() as u32
}

fn part1(contents: &str) -> u32 {
    let map = parse_input(contents);

    let mut total_trails = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 0 {
                total_trails += count_trails(&map, (x, y));
            }
        }
    }

    total_trails
}

fn trail_rating(map: &[Vec<u32>], start: (usize, usize)) -> u32 {
    let height = map.len();
    let width = map[0].len();

    let mut cnt = 0;
    let mut trails = vec![(0u32, start)];
    while let Some(t) = trails.pop() {
        let x = t.1 .0;
        let y = t.1 .1;
        let h = t.0;

        if t.0 == 9 {
            cnt += 1;
            continue;
        }

        if x > 0 && map[y][x - 1] == h + 1 {
            trails.push((h + 1, (x - 1, y)))
        }
        if x < width - 1 && map[y][x + 1] == h + 1 {
            trails.push((h + 1, (x + 1, y)))
        }
        if y < height - 1 && map[y + 1][x] == h + 1 {
            trails.push((h + 1, (x, y + 1)))
        }
        if y > 0 && map[y - 1][x] == h + 1 {
            trails.push((h + 1, (x, y - 1)))
        }
    }

    cnt
}

fn part2(contents: &str) -> u32 {
    let map = parse_input(contents);

    let mut total_trails = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 0 {
                total_trails += trail_rating(&map, (x, y));
            }
        }
    }

    total_trails
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
    fn test_count_trails() {
        let contents = r"0123
1234
8765
9876";
        assert_eq!(part1(contents), 1)
    }

    #[test]
    fn test_count_trails_2() {
        let contents = r"1110111
1111111
1112111
6543456
7111117
8111118
9111119";
        assert_eq!(part1(contents), 2)
    }

    #[test]
    fn test_count_trails_4() {
        let contents = r"1190119
1111198
1112117
6543456
7651987
8761111
9871111";
        assert_eq!(part1(contents), 4)
    }

    #[test]
    fn test_part1() {
        let contents = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part1(contents), 36)
    }

    #[test]
    fn test_trail_rating_3() {
        let contents = r"1111201
1143211
1151121
1165431
1171141
1187651
1191111";
        let map = parse_input(&contents);
        assert_eq!(trail_rating(&map, (5, 0)), 3)
    }

    #[test]
    fn test_part2() {
        let contents = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part2(contents), 81)
    }
}
