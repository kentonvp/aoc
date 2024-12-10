#![allow(dead_code)]

use std::env;
use std::fs;
use regex::Regex;

fn day1_part1(file: &str) -> i32 {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let rows = contents.lines().count();
    let mut list_a: Vec<i32> = Vec::with_capacity(rows);
    let mut list_b: Vec<i32> = Vec::with_capacity(rows);

    for line in contents.lines() {
        let mut cols = line.split_whitespace();
        list_a.push(cols.next().unwrap().parse().unwrap());
        list_b.push(cols.next().unwrap().parse().unwrap());
    }

    list_a.sort();
    list_b.sort();

    let mut distance = 0;
    for i in 0..rows {
        distance += (list_a[i] - list_b[i]).abs();
    }
    distance
}

fn day1_part2(file: &str) -> i32 {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let rows = contents.lines().count();
    let mut list_a: Vec<i32> = Vec::with_capacity(rows);
    let mut list_b: Vec<i32> = Vec::with_capacity(rows);

    for line in contents.lines() {
        let mut cols = line.split_whitespace();
        list_a.push(cols.next().unwrap().parse().unwrap());
        list_b.push(cols.next().unwrap().parse().unwrap());
    }

    let mut similarity = 0;

    // create a hashmap of list_b counts
    let mut map_b = std::collections::HashMap::new();
    for e in list_b.iter() {
        let count = map_b.entry(e).or_insert(0);
        *count += 1;
    }

    for e in list_a.iter() {
        let count = map_b.entry(e).or_insert(0);
        if *count > 0 {
            similarity += *e * *count;
        }
    }

    similarity
}

fn day1(file: &str) {
    let time = std::time::Instant::now();
    let res = day1_part1(&file);
    println!("Part1: {} ({:.4}s)", res, time.elapsed().as_secs_f64());

    let time = std::time::Instant::now();
    let res = day1_part2(&file);
    println!("Part2: {} ({:.4}s)", res, time.elapsed().as_secs_f64());
}

fn check_safety(levels: Vec<i32>) -> bool {
    let mut is_increasing = true;

    for i in 1..levels.len() {
        let diff = levels[i] - levels[i - 1];
        if i == 1 {
            is_increasing = diff > 0;
        }

        if diff == 0 || diff.abs() > 3 || !(is_increasing == (diff > 0)) {
            return false;
        }
    }

    true
}

fn day2_part1(file: &str) -> u32 {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

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

        if diff == 0 || diff.abs() > 3 || !(is_increasing == (diff > 0)) {
            if skip {
                let mut p_levels = levels.clone();
                p_levels.remove(i-1);
                if check_safety(p_levels) {
                    return true;
                }

                let mut n_levels = levels.clone();
                n_levels.remove(i);
                if check_safety(n_levels) {
                    return true
                }

                // skip first element in case the increading/decreasing is wrong
                if i > 1 {
                    let mut f_levels = levels.clone();
                    f_levels.remove(0);
                    if check_safety(f_levels) {
                        return true
                    }
                }
            }
            return false
        }
    }

    true
}

fn day2_part2(file: &str) -> i32 {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

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

fn day2(file: &str) {
    let time = std::time::Instant::now();
    let res = day2_part1(&file);
    println!("Part1: {} ({:.4}s)", res, time.elapsed().as_secs_f64());

    let time = std::time::Instant::now();
    let res = day2_part2(&file);
    println!("Part2: {} ({:.4}s)", res, time.elapsed().as_secs_f64());
}

fn day3_part1(file: &str) -> u32 {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let re = Regex::new(r"mul\((?<f>[0-9]{1,3}),(?<s>[0-9]{1,3})\)").unwrap();

    let muls: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();

    let mut sum = 0;
    muls.iter().for_each(|m| {
        let cap = re.captures(&m).unwrap();
        let f: u32 = cap["f"].parse().unwrap();
        let s: u32 = cap["s"].parse().unwrap();
        sum += f * s;
    });

    sum
}

fn day3_part2(file: &str) -> u32 {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let re = Regex::new(r"mul\((?<f>[0-9]{1,3}),(?<s>[0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();

    let muls: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();

    let mut sum = 0;
    let mut enabled = true;
    for m in muls.iter() {
        if *m == "do()" {
            enabled = true;
        } else if *m == "don't()" {
            enabled = false;
        } else if enabled {
            let cap = re.captures(&m).unwrap();
            let f: u32 = cap["f"].parse().unwrap();
            let s: u32 = cap["s"].parse().unwrap();
            sum += f * s;
        }
    }

    sum
}

fn day3(file: &str) {
    let time = std::time::Instant::now();
    let res = day3_part1(&file);
    println!("Part1: {} ({:.4}s)", res, time.elapsed().as_secs_f64());

    let time = std::time::Instant::now();
    let res = day3_part2(&file);
    println!("Part2: {} ({:.4}s)", res, time.elapsed().as_secs_f64());
}

//fn dayN_part1(file: &str) -> u32 {
//}
//
//fn dayN_part2(file: &str) -> i32 {
//}
//
//fn dayN(file: &str) {
//    let time = std::time::Instant::now();
//    let res = dayN_part1(&file);
//    println!("Part1: {} ({:.4}s)", res, time.elapsed().as_secs_f64());
//
//    let time = std::time::Instant::now();
//    let res = dayN_part2(&file);
//    println!("Part2: {} ({:.4}s)", res, time.elapsed().as_secs_f64());
//}

fn parse_args() -> (u16, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <day> <input>", args[0]);
        std::process::exit(1);
    }
    (args[1].parse().unwrap(), args[2].clone())
}

fn main() {
    let (day, file) = parse_args();
    match day {
        1 => day1(&file),
        2 => day2(&file),
        3 => day3(&file),
        _ => println!("Invalid day"),
    }
}
