pub fn part1(input: &str) -> usize {
    let mut dial = 50;
    let mut zero_counter = 0;
    for l in input.lines() {
        let mut chars = l.chars();
        let dir = chars.next();
        let mut num = l[1..].parse::<u32>().unwrap();

        // count full rotations
        while num >= 100 {
            num -= 100;
        }
        num = if dir == Some('L') { 100 - num } else { num };
        dial = (dial + num) % 100;

        if dial == 0 {
            zero_counter += 1;
        }
    }
    println!("Day 1, Part 1: {}", zero_counter);
    zero_counter
}

pub fn part2(input: &str) -> usize {
    let mut dial = 50;
    let mut zero_counter: usize = 0;
    for l in input.lines() {
        let mut chars = l.chars();
        let dir = chars.next().unwrap();
        let mut num = l[1..].parse::<i32>().unwrap();

        zero_counter += num.div_euclid(100) as usize;
        num %= 100;

        // adjust for direction
        if dir == 'L' {
            num = -num;
        }

        let prev_dial = dial;
        dial += num;
        if dial == 0 && num != 0 {
            // moved to zero NOT from zero
            zero_counter += 1;
        } else if dial < 0 {
            // passed zero going left
            dial += 100;
            if prev_dial != 0 {
                zero_counter += 1;
            }
        } else if dial > 99 {
            // passed zero going right
            dial %= 100;
            zero_counter += 1;
        }

        assert!((0..=99).contains(&dial), "Dial out of bounds: {}", dial);
    }
    println!("Day 1, Part 2: {}", zero_counter);
    zero_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_part2() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;
        assert_eq!(part2(input), 6);
    }
}
