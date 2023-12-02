pub fn process1(input: &str) -> u64 {
    // loop through the lines of a file
    let mut sum = 0u64;
    for line in input.lines() {
        let d1 = line.chars().find(|c| c.is_numeric()).unwrap();
        let d2 = line.chars().rfind(|c| c.is_numeric()).unwrap();

        // parse the concatonated integer
        let d = format!("{}{}", d1, d2);
        sum += d.parse::<u64>().unwrap();
    }
    println!("{sum}");
    return sum;
}

pub fn process2(input: &str) -> u64 {
    let numbers: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut sum = 0u64;
    for line in input.lines() {
        // Map to keep track of first digit/word.
        let mut indices: [usize; 9] = [usize::MAX; 9];
        for i in 0..numbers.len() {
            if let Some(idx) = line.find(numbers[i]) {
                indices[i] = idx;
            }
            if let Some(idx) = line.find(digits[i]) {
                indices[i] = usize::min(idx, indices[i]);
            }
        }
        // get the index of the smallest number in indices
        let mut d1 = 0;
        for i in 0..indices.len() {
            if indices[i] < indices[d1] {
                d1 = i;
            }
        }

        // Map to keep track of last digit/word.
        let mut d_indices: [i32; 9] = [-1; 9];
        for i in 0..numbers.len() {
            if let Some(idx) = line.rfind(numbers[i]) {
                d_indices[i] = idx as i32;
            }
            if let Some(idx) = line.rfind(digits[i]) {
                d_indices[i] = i32::max(idx as i32, d_indices[i]);
            }
        }
        // get the index of the largest number in indices
        let mut d2 = 0;
        for i in 0..d_indices.len() {
            if d_indices[i] > d_indices[d2] {
                d2 = i;
            }
        }

        // parse the contatonated integer
        let d = format!("{}{}", d1 + 1, d2 + 1);
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
    fn test_process1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(process1(input), 142);
    }

    #[test]
    fn test_process2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(process2(input), 281);
    }
}
