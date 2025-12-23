use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let start_idx = lines.next().unwrap().find('S').unwrap();

    let mut total = 0;
    let mut tachyons = HashSet::new();
    tachyons.insert(start_idx);

    while let Some(line) = lines.next() {
        let mut remove_idxs = Vec::new();
        let mut add_idxs = Vec::new();
        for &idx in &tachyons {
            if let Some(c) = line.chars().nth(idx) {
                if c == '^' {
                    total += 1;
                    remove_idxs.push(idx);
                    if idx > 0 {
                        add_idxs.push(idx - 1);
                    }
                    if idx + 1 < line.len() {
                        add_idxs.push(idx + 1);
                    }
                }
            }
        }
        for idx in remove_idxs {
            tachyons.remove(&idx);
        }
        for idx in add_idxs {
            tachyons.insert(idx);
        }
    }
    println!("Day 7, Part 1: {}", total);
    total
}

pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let start_idx = lines.next().unwrap().find('S').unwrap();

    let mut tachyon_paths = std::collections::HashMap::new();
    tachyon_paths.insert(start_idx, 1usize);

    while let Some(line) = lines.next() {
        let mut new_paths = std::collections::HashMap::new();
        for (&current_idx, &num_at) in &tachyon_paths {
            // let &current_idx = tachyon_path.last().unwrap();
            if let Some(c) = line.chars().nth(current_idx) {
                if c == '^' {
                    // split tachyon path
                    if current_idx > 0 {
                        new_paths
                            .entry(current_idx - 1)
                            .and_modify(|e| *e += num_at)
                            .or_insert(num_at);
                    }

                    if current_idx + 1 < line.len() {
                        new_paths
                            .entry(current_idx + 1)
                            .and_modify(|e| *e += num_at)
                            .or_insert(num_at);
                    }
                } else {
                    // continue on the same path: updating entry if exists
                    new_paths
                        .entry(current_idx)
                        .and_modify(|e| *e += num_at)
                        .or_insert(num_at);
                }
            }
        }
        // override tachyon paths with new paths
        tachyon_paths = new_paths
    }

    let total = tachyon_paths.values().sum();
    println!("Day 7, Part 2: {}", total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn test_part2() {
        let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;
        assert_eq!(part2(input), 40);
    }
}
