// Day 7 //////////////////////////////////////////////////////////////////////
fn is_valid_equation(sol: u64, eq: Vec<u64>) -> bool {
    let mut stack = Vec::new();

    stack.push(eq[0]);
    for x2 in eq.into_iter().skip(1) {
        let mut new_stack = Vec::new();
        while let Some(x) = stack.pop() {
            // x * x2
            new_stack.push(x * x2);
            // x + x2
            new_stack.push(x + x2);
        }

        stack = new_stack;
    }

    while let Some(x) = stack.pop() {
        if x == sol {
            return true;
        }
    }

    false
}

fn part1(contents: &str) -> u64 {
    let equations = contents
        .lines()
        .map(|line| {
            // split by whitespace
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let sol = parts[0][..parts[0].len() - 1].parse::<u64>().unwrap();
            let eq = parts[1..]
                .iter()
                .map(|p| p.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (sol, eq)
        })
        .collect::<Vec<(u64, Vec<u64>)>>();

    let mut sum_test_values = 0;
    for (sol, eq) in equations {
        if is_valid_equation(sol, eq) {
            sum_test_values += sol;
        }
    }

    sum_test_values
}

fn is_valid_equation_p2(sol: u64, eq: Vec<u64>) -> bool {
    let mut stack = Vec::new();

    // Ex.
    // 7290 = 6 8 6 15
    //      = 6 * 8 || 6 * 15
    //      = 48 || 6 * 15
    //      = 486 * 15
    //      = 7290
    stack.push(eq[0]);
    for x2 in eq.into_iter().skip(1) {
        let mut new_stack = Vec::new();
        while let Some(x) = stack.pop() {
            // x * x2
            new_stack.push(x * x2);
            // x + x2
            new_stack.push(x + x2);
            // x || x2
            new_stack.push(
                format!("{x}{x2}").parse::<u64>().expect("unable to concat"),
            );
        }

        stack = new_stack;
    }

    while let Some(x) = stack.pop() {
        if x == sol {
            return true;
        }
    }

    false
}

fn part2(contents: &str) -> u64 {
    let equations = contents
        .lines()
        .map(|line| {
            // split by whitespace
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let sol = parts[0][..parts[0].len() - 1].parse::<u64>().unwrap();
            let eq = parts[1..]
                .iter()
                .map(|p| p.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (sol, eq)
        })
        .collect::<Vec<(u64, Vec<u64>)>>();

    let mut sum_test_values = 0;
    for (sol, eq) in equations {
        if is_valid_equation_p2(sol, eq) {
            sum_test_values += sol;
        }
    }

    sum_test_values
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
        let contents = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part1(contents), 3749)
    }

    #[test]
    fn test_part2() {
        let contents = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part2(contents), 11387)
    }

    #[test]
    fn test_is_valid_equation_p2() {
        assert!(is_valid_equation_p2(156, vec![15, 6]));
        assert!(is_valid_equation_p2(7290, vec![6, 8, 6, 15]));
        assert!(is_valid_equation_p2(192, vec![17, 8, 14]));
    }
}
