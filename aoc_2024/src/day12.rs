// Day 12 /////////////////////////////////////////////////////////////////////
use std::collections::HashSet;

fn parse_map(contents: &str) -> Vec<Vec<char>> {
    contents.lines().map(|l| l.chars().collect()).collect()
}

fn explore_neighbors(
    map: &[Vec<char>],
    visited: &mut [Vec<bool>],
    start: (usize, usize),
) -> HashSet<(usize, usize)> {
    let width = map.len();
    let height = map[0].len();

    let mut plot = HashSet::new();
    let plot_id = map[start.1][start.0];

    let mut visit = vec![start];
    while let Some(p) = visit.pop() {
        let (x, y) = p;
        if visited[y][x] {
            continue;
        }
        if map[y][x] != plot_id {
            continue;
        }
        visited[y][x] = true;
        plot.insert(p);

        // Visit neighbors
        if x > 0 {
            visit.push((x - 1, y));
        }
        if x < width - 1 {
            visit.push((x + 1, y));
        }
        if y > 0 {
            visit.push((x, y - 1));
        }
        if y < height - 1 {
            visit.push((x, y + 1));
        }
    }

    plot
}

fn perimeter(plot: &HashSet<(usize, usize)>) -> u32 {
    let mut cnt = 0;

    for &(x, y) in plot {
        if x == 0 || !plot.contains(&(x - 1, y)) {
            cnt += 1;
        }
        if y == 0 || !plot.contains(&(x, y - 1)) {
            cnt += 1;
        }
        if !plot.contains(&(x + 1, y)) {
            cnt += 1;
        }
        if !plot.contains(&(x, y + 1)) {
            cnt += 1;
        }
    }

    cnt
}

fn area(plot: &HashSet<(usize, usize)>) -> u32 {
    plot.len() as u32
}

fn part1(contents: &str) -> u32 {
    let map = parse_map(contents);

    // Create a log of visited cells
    let mut visited = Vec::with_capacity(map.len());

    for row in map.iter() {
        visited.push(vec![false; row.len()]);
    }

    // Keep track of plots
    let mut fence_cost = 0;

    // Visit all cells adding them to plots
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if visited[y][x] {
                continue;
            }
            let plot = explore_neighbors(&map, &mut visited, (x, y));
            fence_cost += area(&plot) * perimeter(&plot);
        }
    }

    fence_cost
}

fn part2(contents: &str) -> u32 {
    todo!()
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
    fn test_part1_example1() {
        let contents = r"AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part1(contents), 140)
    }

    #[test]
    fn test_part1_example2() {
        let contents = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part1(contents), 772)
    }

    #[test]
    fn test_part1() {
        let contents = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part1(contents), 1930)
    }

    #[test]
    fn test_part2_example1() {
        let contents = r"AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part1(contents), 80)
    }

    #[test]
    fn test_part2_example2() {
        let contents = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part1(contents), 436)
    }

    #[test]
    fn test_part2() {
        let contents = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part1(contents), 1206)
    }
}
