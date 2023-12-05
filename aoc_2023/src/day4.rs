use std::collections::HashSet;

pub fn process1(input: &str) -> u32 {
    let stime = std::time::Instant::now();

    let sum = input
        .lines()
        .map(|line| {
            let sets = line
                .split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .map(|set| {
                    set.split(' ')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<HashSet<u32>>()
                })
                .collect::<Vec<HashSet<u32>>>();
            let winning = sets[0].intersection(&sets[1]).collect::<Vec<&u32>>().len();
            if winning == 0 {
                0
            } else {
                2u32.pow(winning as u32 - 1)
            }
        })
        .sum::<u32>();

    println!("Day 4 - Part 1: {} [{:?}]", sum, stime.elapsed());
    sum
}

pub fn process2(input: &str) -> u32 {
    let stime = std::time::Instant::now();

    let card_copies = input
        .lines()
        .map(|line| {
            let sets = line
                .split(':')
                .nth(1)
                .unwrap()
                .split('|')
                .map(|set| {
                    set.split(' ')
                        .map(|s| s.trim())
                        .filter(|s| !s.is_empty())
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<HashSet<u32>>()
                })
                .collect::<Vec<HashSet<u32>>>();
            sets[0].intersection(&sets[1]).collect::<Vec<&u32>>().len()
        })
        .collect::<Vec<usize>>();

    let mut counter = Vec::new();
    counter.resize(card_copies.len(), 1u32);
    let mut sum = 0;
    for (i, n_cards) in card_copies.iter().enumerate() {
        sum += counter[i];
        for j in i + 1..=i + *n_cards {
            counter[j] += counter[i];
        }
    }

    println!("Day 4 - Part 2: {} [{:?}]", sum, stime.elapsed());
    sum
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process1(input), 13)
    }

    #[test]
    fn test_process2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process2(input), 30)
    }
}