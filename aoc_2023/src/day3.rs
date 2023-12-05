fn is_symbol(c: char) -> bool {
    "@#$%&*+=-/".contains(c)
}

fn is_gear(c: char) -> bool {
    c == '*'
}

pub fn process1(input: &str) -> u32 {
    let stime = std::time::Instant::now();

    let mut sum = 0u32;
    let mut prev_i: Option<usize> = None;
    for (i, line) in input.lines().enumerate() {
        let mut pointer = 0;
        let mut is_adjacent: bool;

        while pointer < line.len() {
            is_adjacent = false;
            let segment = &line[pointer..];
            // Look for the first digit in the segment.
            let num_sidx = segment.find(|c: char| c.is_ascii_digit());

            // no digit found
            if num_sidx.is_none() {
                break;
            }

            let num_sidx = num_sidx.unwrap();
            // Look for the end of the digit in the segment.
            let rest = &segment[num_sidx..];
            let num_eidx = rest
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(rest.len())
                + num_sidx;

            let number = &segment[num_sidx..num_eidx].parse::<u32>().unwrap();

            let l_sidx = if pointer + num_sidx > 0 {
                pointer + num_sidx - 1
            } else {
                0
            };

            let l_eidx = line.len().min(pointer + num_eidx + 1);

            // Check current row.
            is_adjacent |= line.get(l_sidx..l_eidx).unwrap().chars().any(is_symbol);

            // Previous row.
            if let Some(i_m1) = prev_i {
                is_adjacent |= input
                    .lines()
                    .nth(i_m1)
                    .unwrap()
                    .get(l_sidx..l_eidx)
                    .unwrap()
                    .chars()
                    .any(is_symbol);
            }

            // Next row.
            if let Some(next_line) = input.lines().nth(i + 1) {
                is_adjacent |= next_line
                    .get(l_sidx..l_eidx)
                    .unwrap()
                    .chars()
                    .any(is_symbol);
            }

            if is_adjacent {
                sum += number;
            }

            pointer += num_eidx;
        }
        prev_i = Some(i);
    }
    println!("Day 3 - Part 1: {} [{:?}]", sum, stime.elapsed());
    sum
}

pub fn process2(input: &str) -> u32 {
    let stime = std::time::Instant::now();

    let mut sum = 0u32;
    let mut prev_i: Option<usize> = None;
    let mut gear_map: std::collections::HashMap<u32, Vec<u32>> = std::collections::HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let n = line.len();
        let mut pointer = 0;

        while pointer < line.len() {
            let segment = &line[pointer..];
            // Look for the first digit in the segment.
            let num_sidx = segment.find(|c: char| c.is_ascii_digit());

            // no digit found
            if num_sidx.is_none() {
                break;
            }

            let num_sidx = num_sidx.unwrap();
            // Look for the end of the digit in the segment.
            let rest = &segment[num_sidx..];
            let num_eidx = rest
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(rest.len())
                + num_sidx;

            let number = &segment[num_sidx..num_eidx].parse::<u32>().unwrap();

            let l_sidx = if pointer + num_sidx > 0 {
                pointer + num_sidx - 1
            } else {
                0
            };

            let l_eidx = line.len().min(pointer + num_eidx + 1);

            // Check current row.
            let g_idx = line.get(l_sidx..l_eidx).unwrap().chars().position(is_gear);
            if let Some(g_idx) = g_idx {
                let gear_id = (n * i) + (g_idx + l_sidx);
                let gear = gear_map.entry(gear_id as u32).or_default();
                gear.push(*number);
            }

            // Previous row.
            if let Some(i_m1) = prev_i {
                let g_idx = input
                    .lines()
                    .nth(i_m1)
                    .unwrap()
                    .get(l_sidx..l_eidx)
                    .unwrap()
                    .chars()
                    .position(is_gear);
                if let Some(g_idx) = g_idx {
                    let gear_id = (n * i_m1) + (g_idx + l_sidx);
                    let gear = gear_map.entry(gear_id as u32).or_default();
                    gear.push(*number);
                }
            }

            // Next row.
            if let Some(next_line) = input.lines().nth(i + 1) {
                let g_idx = next_line
                    .get(l_sidx..l_eidx)
                    .unwrap()
                    .chars()
                    .position(is_gear);
                if let Some(g_idx) = g_idx {
                    let gear_id = (n * (i + 1)) + (g_idx + l_sidx);
                    let gear = gear_map.entry(gear_id as u32).or_default();
                    gear.push(*number);
                }
            }

            pointer += num_eidx;
        }
        prev_i = Some(i);
    }

    for gear in gear_map.values() {
        if gear.len() == 2 {
            sum += gear.iter().product::<u32>();
        }
    }

    println!("Day 3 - Part 2: {} [{:?}]", sum, stime.elapsed());
    sum
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(process1(input), 4361)
    }

    #[test]
    fn test_process2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(process2(input), 467835)
    }
}
