// Day 1 /////////////////////////////////////////////////////////////////////
fn part1(contents: &str) -> i32 {
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

fn part2(contents: &str) -> i32 {
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

pub fn solve(contents: &str) {
    let time = std::time::Instant::now();
    let res = part1(contents);
    println!("Part1: {} ({:.4}s)", res, time.elapsed().as_secs_f64());

    let time = std::time::Instant::now();
    let res = part2(contents);
    println!("Part2: {} ({:.4}s)", res, time.elapsed().as_secs_f64());
}
