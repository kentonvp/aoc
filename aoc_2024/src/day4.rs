// Day 4 /////////////////////////////////////////////////////////////////////
#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn check_north(grid: &[Vec<char>], p: &Point) -> u32 {
    if p.x >= 3
        && grid[p.x - 1][p.y] == 'M'
        && grid[p.x - 2][p.y] == 'A'
        && grid[p.x - 3][p.y] == 'S'
    {
        1
    } else {
        0
    }
}

fn check_northeast(grid: &[Vec<char>], p: &Point) -> u32 {
    if p.x >= 3
        && p.y < grid.len() - 3
        && grid[p.x - 1][p.y + 1] == 'M'
        && grid[p.x - 2][p.y + 2] == 'A'
        && grid[p.x - 3][p.y + 3] == 'S'
    {
        1
    } else {
        0
    }
}

fn check_east(grid: &[Vec<char>], p: &Point) -> u32 {
    if p.y < grid.len() - 3
        && grid[p.x][p.y + 1] == 'M'
        && grid[p.x][p.y + 2] == 'A'
        && grid[p.x][p.y + 3] == 'S'
    {
        1
    } else {
        0
    }
}

fn check_southeast(grid: &[Vec<char>], p: &Point) -> u32 {
    if p.x < grid.len() - 3
        && p.y < grid.len() - 3
        && grid[p.x + 1][p.y + 1] == 'M'
        && grid[p.x + 2][p.y + 2] == 'A'
        && grid[p.x + 3][p.y + 3] == 'S'
    {
        1
    } else {
        0
    }
}

fn check_south(grid: &[Vec<char>], p: &Point) -> u32 {
    if p.x < grid.len() - 3
        && grid[p.x + 1][p.y] == 'M'
        && grid[p.x + 2][p.y] == 'A'
        && grid[p.x + 3][p.y] == 'S'
    {
        1
    } else {
        0
    }
}

fn check_southwest(grid: &[Vec<char>], p: &Point) -> u32 {
    if p.x < grid.len() - 3
        && p.y >= 3
        && grid[p.x + 1][p.y - 1] == 'M'
        && grid[p.x + 2][p.y - 2] == 'A'
        && grid[p.x + 3][p.y - 3] == 'S'
    {
        1
    } else {
        0
    }
}

fn check_west(grid: &[Vec<char>], p: &Point) -> u32 {
    if p.y >= 3
        && grid[p.x][p.y - 1] == 'M'
        && grid[p.x][p.y - 2] == 'A'
        && grid[p.x][p.y - 3] == 'S'
    {
        1
    } else {
        0
    }
}

fn check_northwest(grid: &[Vec<char>], p: &Point) -> u32 {
    if p.x >= 3
        && p.y >= 3
        && grid[p.x - 1][p.y - 1] == 'M'
        && grid[p.x - 2][p.y - 2] == 'A'
        && grid[p.x - 3][p.y - 3] == 'S'
    {
        1
    } else {
        0
    }
}

fn part1(contents: &str) -> u32 {
    // Populate grid of characters
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    // Find all possible starting points 'X'
    let mut starts: Vec<Point> = Vec::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, e) in row.iter().enumerate() {
            if *e == 'X' {
                starts.push(Point { x, y });
            }
        }
    }

    // For each starting point check if following a direction we can spell the word 'XMAS'
    // check right, left, down, up, and all diagonals
    let mut found = 0;
    for p in starts.iter() {
        found += check_north(&grid, p);
        found += check_northeast(&grid, p);
        found += check_east(&grid, p);
        found += check_southeast(&grid, p);
        found += check_south(&grid, p);
        found += check_southwest(&grid, p);
        found += check_west(&grid, p);
        found += check_northwest(&grid, p);
    }

    found
}

fn check_diagonal(grid: &[Vec<char>], p: &Point) -> u32 {
    if p.x > 0
        && p.x < grid.len() - 1
        && p.y > 0
        && p.y < grid[p.x].len() - 1
        && ((grid[p.x - 1][p.y - 1] == 'M'
            && grid[p.x + 1][p.y - 1] == 'M'
            && grid[p.x - 1][p.y + 1] == 'S'
            && grid[p.x + 1][p.y + 1] == 'S')
            || (grid[p.x - 1][p.y - 1] == 'S'
                && grid[p.x + 1][p.y - 1] == 'M'
                && grid[p.x - 1][p.y + 1] == 'S'
                && grid[p.x + 1][p.y + 1] == 'M')
            || (grid[p.x - 1][p.y - 1] == 'S'
                && grid[p.x + 1][p.y - 1] == 'S'
                && grid[p.x - 1][p.y + 1] == 'M'
                && grid[p.x + 1][p.y + 1] == 'M')
            || (grid[p.x - 1][p.y - 1] == 'M'
                && grid[p.x + 1][p.y - 1] == 'S'
                && grid[p.x - 1][p.y + 1] == 'M'
                && grid[p.x + 1][p.y + 1] == 'S'))
    {
        1
    } else {
        0
    }
}

fn part2(contents: &str) -> u32 {
    // Populate grid of characters
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    // Find all possible starting points 'A'
    let mut starts: Vec<Point> = Vec::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, e) in row.iter().enumerate() {
            if *e == 'A' {
                starts.push(Point { x, y });
            }
        }
    }

    let mut found = 0;
    for p in starts.iter() {
        found += check_diagonal(&grid, p);
    }

    found
}

pub fn solve(contents: &str) {
    let time = std::time::Instant::now();
    let res = part1(contents);
    println!("Part1: {} ({:.4}s)", res, time.elapsed().as_secs_f64());

    let time = std::time::Instant::now();
    let res = part2(contents);
    println!("Part2: {} ({:.4}s)", res, time.elapsed().as_secs_f64());
}
