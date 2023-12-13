fn expand_sample(input: &str) -> Vec<Vec<i64>> {
    let sample: Vec<i64> = input.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();

    let mut expanded: Vec<Vec<i64>> = Vec::new();
    expanded.push(sample);
    
    while !expanded.last().unwrap().iter().all(|&x| x == 0) {
        let mut next: Vec<i64> = Vec::new();
        let last = expanded.last().unwrap();
        for i in 1..last.len() {
            next.push(last[i] - last[i-1]);
        }
        expanded.push(next);
    }

    expanded
}

fn extrapolate_next(mut expanded: Vec<Vec<i64>>) -> i64 {
    // Pop off all zero vector.
    expanded.pop();

    let mut delta = 0;
    while let Some(prev) = expanded.pop() {
        delta += prev[prev.len()-1];
    }
    delta
}

fn extrapolate_prev(mut expanded: Vec<Vec<i64>>) -> i64 {
    // Pop off all zero vector.
    expanded.pop();

    let mut delta = 0;
    while let Some(prev) = expanded.pop() {
        delta = prev[0] - delta;
    }
    delta
}

pub fn process1(input: &str) -> i64 {
    let stime = std::time::Instant::now();
    let sol = input.lines().map(|l| {
        let expanded = expand_sample(l);
        extrapolate_next(expanded)
    }).sum();
    println!("Day 9 - Part 1: {} [{:?}]", sol, stime.elapsed());
    sol
}

pub fn process2(input: &str) -> i64 {
    let stime = std::time::Instant::now();
    let sol = input.lines().map(|l| {
        let expanded = expand_sample(l);
        extrapolate_prev(expanded)
    }).sum();
    println!("Day 9 - Part 2: {} [{:?}]", sol, stime.elapsed());
    sol
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(process1(input), 114)
    }

    #[test]
    fn test_process2() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(process2(input), 2)
    }
}
