// Day 2 /////////////////////////////////////////////////////////////////////
fn check_safety(levels: Vec<i32>) -> bool {
    let mut is_increasing = true;

    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        if i == 1 {
            is_increasing = diff > 0;
        }

        if diff == 0 || diff.abs() > 3 || is_increasing != (diff > 0) {
            return false;
        }
    }

    true
}

fn part1(contents: &str) -> u32 {
    let mut safe_counts = 0;
    for line in contents.lines() {
        let cols: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if check_safety(cols) {
            safe_counts += 1;
        }
    }

    safe_counts
}

fn check_safety_with_skip(levels: Vec<i32>, skip: bool) -> bool {
    let mut is_increasing = true;

    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        if i == 1 {
            is_increasing = diff > 0;
        }

        if diff == 0 || diff.abs() > 3 || is_increasing != (diff > 0) {
            if skip {
                let mut p_levels = levels.clone();
                p_levels.remove(i - 1);
                if check_safety(p_levels) {
                    return true;
                }

                let mut n_levels = levels.clone();
                n_levels.remove(i);
                if check_safety(n_levels) {
                    return true;
                }

                // skip first element in case the increading/decreasing is wrong
                if i > 1 {
                    let mut f_levels = levels.clone();
                    f_levels.remove(0);
                    if check_safety(f_levels) {
                        return true;
                    }
                }
            }
            return false;
        }
    }

    true
}

fn part2(contents: &str) -> i32 {
    let mut safe_counts = 0;
    for line in contents.lines() {
        let cols: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if check_safety_with_skip(cols, true) {
            safe_counts += 1;
        }
    }

    safe_counts
}

pub fn solve(contents: &str) {
    let time = std::time::Instant::now();
    let res = part1(contents);
    println!("Part1: {} ({:.4}s)", res, time.elapsed().as_secs_f64());

    let time = std::time::Instant::now();
    let res = part2(contents);
    println!("Part2: {} ({:.4}s)", res, time.elapsed().as_secs_f64());
}
