// Day 7 //////////////////////////////////////////////////////////////////////
// Trick is to track the number using bits. At each operator position it is
// binary. Which means we can track using bits.
// Simplest example with only 1 operator:
// a * b 0
// a + b 1
//
// 2 operator:
// a * b * c 00
// a * b + c 01
// a + b + c 11
// a + b * c 10
//
// 3 operator:
// a * b * c * d 000
// a * b * c + d 001
// a * b + c * d 010
// a * b + c + d 011
// a + b * c * d 100
// a + b * c + d 101
// a + b + c * d 110
// a + b + c + d 111
fn is_valid_equation(sol: u64, eq: Vec<u64>) -> bool {
    // Number of possible operator permuations is 2^(n-1).
    let n_opts = 2u64.pow((eq.len() - 1) as u32);

    for i in 0..n_opts {
        let mut answer = eq[0];
        for pos in 0..eq.len() - 1 {
            // Check if bit at position `pos` in `i` is set.
            if (i >> pos) & 1 == 1 {
                answer += eq[pos + 1];
            } else {
                answer *= eq[pos + 1];
            }

            // Since answer is always increasing, we can early exit if the
            // current answer is greater than the expected solution.
            if answer > sol {
                break;
            }
        }

        // Found correct answer, can early exit.
        if answer == sol {
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

fn part2(contents: &str) -> u64 {
    todo!()
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
}
