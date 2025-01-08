use regex::Regex;

// Day 3 /////////////////////////////////////////////////////////////////////
fn part1(contents: &str) -> u32 {
    let re = Regex::new(r"mul\((?<f>[0-9]{1,3}),(?<s>[0-9]{1,3})\)").unwrap();

    let muls: Vec<&str> = re.find_iter(contents).map(|m| m.as_str()).collect();

    let mut sum = 0;
    muls.iter().for_each(|m| {
        let cap = re.captures(m).unwrap();
        let f: u32 = cap["f"].parse().unwrap();
        let s: u32 = cap["s"].parse().unwrap();
        sum += f * s;
    });

    sum
}

fn part2(contents: &str) -> u32 {
    let re = Regex::new(
        r"mul\((?<f>[0-9]{1,3}),(?<s>[0-9]{1,3})\)|do\(\)|don't\(\)",
    )
    .unwrap();

    let muls: Vec<&str> = re.find_iter(contents).map(|m| m.as_str()).collect();

    let mut sum = 0;
    let mut enabled = true;
    for m in muls.iter() {
        if *m == "do()" {
            enabled = true;
        } else if *m == "don't()" {
            enabled = false;
        } else if enabled {
            let cap = re.captures(m).unwrap();
            let f: u32 = cap["f"].parse().unwrap();
            let s: u32 = cap["s"].parse().unwrap();
            sum += f * s;
        }
    }

    sum
}

pub fn solve(contents: &str) {
    let time = std::time::Instant::now();
    let res = part1(contents);
    println!("Part1: {} ({:.4}s)", res, time.elapsed().as_secs_f64());

    let time = std::time::Instant::now();
    let res = part2(contents);
    println!("Part2: {} ({:.4}s)", res, time.elapsed().as_secs_f64());
}
