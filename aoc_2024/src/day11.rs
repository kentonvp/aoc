// Day 11 /////////////////////////////////////////////////////////////////////
use std::collections::HashMap;

fn parse_input(contents: &str) -> Vec<u64> {
    contents.split_whitespace().map(|c| c.parse().unwrap()).collect()
}

fn has_even_digits(s: u64) -> bool {
    s.to_string().len() % 2 == 0
}

fn split_digits(s: u64) -> (u64, u64) {
    let s = s.to_string();
    let n = s.len() / 2;

    let p1 = s[..n].parse().unwrap();
    let p2 = s[n..].parse().unwrap();
    (p1, p2)
}

fn blink(stone: u64) -> (u64, Option<u64>) {
    match stone {
        s if s == 0 => (1, None),
        s if has_even_digits(s) => {
            let (p1, p2) = split_digits(s);
            (p1, Some(p2))
        }
        s => (s * 2024, None),
    }
}

fn count_stone_blinks(stone: u64, n_blinks: usize, cache: &mut HashMap::<(u64, usize), usize>, blink_cache: &mut HashMap::<u64, (u64, Option<u64>)>) -> usize {
    if let Some(&out) = cache.get(&(stone, n_blinks)) {
        return out
    }

    let (p1, p2) = match blink_cache.get(&stone) {
        Some((p1, p2)) => (*p1, *p2),
        None => {
            let (p1, p2) = blink(stone);
            blink_cache.insert(stone, (p1, p2));
            (p1, p2)
        }
    };

    if n_blinks == 1 {
        if p2.is_some() {
            cache.insert((stone, 1), 2);
            return 2
        } else {
            cache.insert((stone, 1), 1);
            return 1
        }
    }

    let mut out = count_stone_blinks(p1, n_blinks -1, cache, blink_cache);
    
    if let Some(p2) = p2 {
        out += count_stone_blinks(p2, n_blinks - 1, cache, blink_cache)
    }

    cache.insert((stone, n_blinks), out);
    out
}

fn part1(contents: &str) -> usize {
    let stones = parse_input(contents);
    let mut out = 0;
    let mut cache = HashMap::new();
    let mut b_cache = HashMap::new();
    for s in stones {
        out += count_stone_blinks(s, 25, &mut cache, &mut b_cache);
    }
    out
}

fn part2(contents: &str) -> usize {
    let stones = parse_input(contents);
    let mut out = 0;
    let mut cache = HashMap::new();
    let mut b_cache = HashMap::new();
    for s in stones {
        out += count_stone_blinks(s, 75, &mut cache, &mut b_cache);
    }
    out
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
        let contents = r"125 17";
        assert_eq!(part1(contents), 55312)
    }

    #[test]
    fn test_part2() {
        let contents = r"125 17";
        assert_eq!(part2(contents), 65601038650482)
    }
}
