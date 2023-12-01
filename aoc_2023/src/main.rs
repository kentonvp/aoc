fn main() {
    dotenv::dotenv().ok();
    let session = std::env::var("AOC_SESSION_TOKEN").expect("AOC_SESSION_TOKEN not set");

    let fname = download_input(1, "a", &session);

    let input = std::fs::read_to_string(fname).unwrap();

    day1(input.clone());
    day1b(input.clone());
}

fn download_input(day: u8, part: &str, session: &str) -> String {
    let input_file = format!("inputs/day{}{}.txt", day, part);
    if std::path::Path::new(&input_file).exists() {
        println!("{} already exists", input_file);
        return input_file;
    }

    let url = format!("https://adventofcode.com/2023/day/{}/input", day);

    let client = reqwest::blocking::Client::new();
    let res = client.get(&url)
        .header("Cookie", format!("session={}", session))
        .send().unwrap();

    let body = res.text().unwrap();
    std::fs::write(&input_file, body).unwrap();

    println!("{}", input_file);
    return input_file;
}

fn day1(input: String) -> u64 {
    // loop through the lines of a file
    let mut sum = 0u64;
    for line in input.lines() {
        let s = line.to_string();
        // find the first numeric character of the line
        let d1_idx = s.find(char::is_numeric).unwrap();
        let d1 = s.get(d1_idx..d1_idx+1).unwrap();

        let d2_idx = s.rfind(char::is_numeric).unwrap();
        let d2 = s.get(d2_idx..d2_idx+1).unwrap();
        let d = format!("{}{}", d1, d2);

        // parse d to integer
        sum += d.parse::<u64>().unwrap();
    }
    println!("{sum}");
    return sum;
}

fn day1b(input: String) -> u64 {
    let numbers: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut sum = 0u64;
    for line in input.lines() {
        // Map to keep track of first digit/word.
        let mut indices: [usize; 10] = [usize::MAX; 10];
        for i in 0..numbers.len() {
            if line.contains(numbers[i]) {
                indices[i] = line.find(numbers[i]).unwrap();
            }
            if line.contains(digits[i]) {
                let d_idx = line.find(digits[i]).unwrap();
                indices[i] = usize::min(d_idx, indices[i]);
            }
        }
        // get the index of the smallest number in indices
        let mut d1 = 0;
        for i in 1..indices.len() {
            if indices[i] < indices[d1] {
                d1 = i;
            }
        }

        // Map to keep track of last digit/word.
        let mut indices: [i32; 10] = [-1; 10];
        for i in 0..numbers.len() {
            if line.contains(numbers[i]) {
                indices[i] = line.rfind(numbers[i]).unwrap() as i32;
            }
            if line.contains(digits[i]) {
                let d_idx = line.rfind(digits[i]).unwrap() as i32;
                indices[i] = i32::max(d_idx, indices[i]);
            }
        }
        // get the index of the largest number in indices
        let mut d2 = 0;
        for i in 1..indices.len() {
            if indices[i] > indices[d2] {
                d2 = i;
            }
        }

        let d = format!("{}{}", d1, d2);

        // parse d to integer
        sum += d.parse::<u64>().unwrap();
    }
    println!("{sum}");
    return sum;
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let input = String::from("1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet");
        assert!(day1(input) == 142);
    }

    #[test]
    fn test_day1b() {
        let input = String::from("two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen");
        assert!(day1b(input) == 281);
    }
}
