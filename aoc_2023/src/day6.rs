fn ways_to_beat_record(time: u64, record: u64) -> u64 {
    (1..time).filter(|t| t * (time - t) > record).count() as u64
}

pub fn process1(input: &str) -> u64 {
    let stime = std::time::Instant::now();

    let times = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let distances = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut sol = Vec::new();
    for i in 0..times.len() {
        sol.push(ways_to_beat_record(times[i], distances[i]));
    }

    let sol: u64 = sol.into_iter().product();

    println!("Day 6 - Part 1: {} [{:?}]", sol, stime.elapsed());
    sol
}

pub fn process2(input: &str) -> u64 {
    let stime = std::time::Instant::now();

    let time = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let distance = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let sol = ways_to_beat_record(time, distance);

    println!("Day 6 - Part 2: {} [{:?}]", sol, stime.elapsed());
    sol
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process1(input), 288)
    }

    #[test]
    fn test_process2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process2(input), 71503)
    }
}
