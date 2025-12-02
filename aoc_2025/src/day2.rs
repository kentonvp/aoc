use fancy_regex::Regex;

fn is_repeating(s: &str, multiple: bool) -> bool {
    if !multiple {
        let re = Regex::new(r"^(.*)\1{1}$").unwrap();
        re.is_match(s).unwrap()
    } else {
        let re = Regex::new(r"^(.*)\1+$").unwrap();
        re.is_match(s).unwrap()
    }
}

pub fn part1(input: &str) -> usize {
    let input = input.trim();
    let ranges = input.split(',');

    let mut total = 0;
    for rng in ranges {
        let mut bounds = rng.splitn(2, '-');
        let lower = bounds.next().unwrap().parse::<u64>().unwrap();
        let upper = bounds.next().unwrap().parse::<u64>().unwrap();

        for num in lower..=upper {
            let str_num = format!("{}", num);
            if str_num.len() % 2 != 0 {
                continue;
            }
            if is_repeating(&format!("{}", num), false) {
                total += num as usize;
            }
        }
    }

    println!("Part 1: {}", total);
    total
}

pub fn part2(input: &str) -> usize {
    let input = input.trim();
    let ranges = input.split(',');

    let mut total = 0;
    for rng in ranges {
        let mut bounds = rng.splitn(2, '-');
        let lower = bounds.next().unwrap().parse::<u64>().unwrap();
        let upper = bounds.next().unwrap().parse::<u64>().unwrap();

        for num in lower..=upper {
            if is_repeating(&format!("{}", num), true) {
                total += num as usize;
            }
        }
    }

    println!("Part 2: {}", total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;
        assert_eq!(part1(input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;
        assert_eq!(part2(input), 4174379265);
    }
}
