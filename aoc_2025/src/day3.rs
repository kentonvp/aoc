pub fn part1(input: &str) -> usize {
    let mut total = 0;

    for l in input.lines() {
        let digits = l
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();

        // Find the max digit in the bank.
        let mut max_idx: Option<usize> = None;
        let mut max_left = 0;
        for i in 0..digits.len() - 1 {
            let d = digits[i];
            if d > max_left {
                max_left = d;
                max_idx = Some(i);
            }
        }

        let max_idx = max_idx.expect("always be at least one digit");

        let mut max_right = 0;
        // Find the right max digit
        for i in max_idx + 1..digits.len() {
            let d = digits[i];
            if d > max_right {
                max_right = d;
            }
        }

        total += format!("{}{}", max_left, max_right)
            .parse::<usize>()
            .unwrap();
    }

    println!("Day 3, Part 1: {}", total);
    total
}

pub fn part2(input: &str) -> usize {
    let mut total = 0;

    for l in input.lines() {
        let digits = l
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();

        let mut jolts = [0; 12];

        let mut start_idx = 0;
        for i in 0..12 {
            let mut max = 0;
            let mut max_idx = 0;
            for j in start_idx..digits.len() - (11 - i) {
                let d = digits[j];
                if d > max {
                    max = d;
                    max_idx = j;
                }
            }

            jolts[i] = digits[max_idx];
            start_idx = max_idx + 1;
        }

        // Turn jolts into a string
        let mut str_jolts = String::new();
        for jolt in jolts.iter() {
            str_jolts.push_str(&jolt.to_string());
        }

        total += str_jolts.parse::<usize>().unwrap();
    }

    println!("Day 3, Part 2: {}", total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        assert_eq!(part1(input), 357);
    }

    #[test]
    fn test_part2() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        assert_eq!(part2(input), 3121910778619);
    }
}
