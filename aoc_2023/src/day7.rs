use std::cmp::Ordering;

#[derive(Eq, Debug)]
struct Hand {
    cards: [u8; 5],
    bid: u32,
}

impl Hand {
    fn get_handtype(&self) -> u8 {
        let mut counts = [0u8; 15];
        for i in 0..5 {
            counts[self.cards[i] as usize] += 1;
        }

        // Jokers are at index 0.
        let jokers = counts[0];

        // Remove jokers from counts.
        counts[0] = 0;

        if counts.iter().any(|&c| c == 5) || jokers == 5 {
            // Five of a kind.
            return 7;
        } else if counts.iter().any(|&c| c == 4) {
            // Four of a kind.
            match jokers {
                0 => return 6,
                1 => return 7,
                _ => panic!("Invalid hand {:?}", self),
            }
        } else if counts.iter().any(|&c| c == 3) && counts.iter().any(|&c| c == 2) {
            // Full house.
            return 5;
        } else if counts.iter().any(|&c| c == 3) {
            // Three of a kind.
            match jokers {
                0 => return 4,
                1 => return 6,
                2 => return 7,
                _ => panic!("Invalid hand {:?}", self),
            }
        } else if counts.iter().filter(|&&c| c == 2).count() == 2 {
            // Two pair.
            match jokers {
                0 => return 3,
                1 => return 5,
                _ => panic!("Invalid hand {:?}", self),
            }
        } else if counts.iter().any(|&c| c == 2) {
            // One pair.
            match jokers {
                0 => return 2,
                1 => return 4,
                2 => return 6,
                3 => return 7,
                _ => panic!("Invalid hand {:?}", self),
            }
        } else if counts.iter().all(|&c| c < 2) {
            // High card.
            match jokers {
                0 => return 1,
                1 => return 2,
                2 => return 4,
                3 => return 6,
                4 => return 7,
                _ => panic!("Invalid hand {:?}", self),
            }
        } else {
            panic!("Invalid hand {:?}", self)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.get_handtype() < other.get_handtype() {
            return std::cmp::Ordering::Less;
        } else if self.get_handtype() > other.get_handtype() {
            return std::cmp::Ordering::Greater;
        }

        for i in 0..5 {
            if self.cards[i] < other.cards[i] {
                return std::cmp::Ordering::Less;
            } else if self.cards[i] > other.cards[i] {
                return std::cmp::Ordering::Greater;
            }
        }
        return std::cmp::Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.get_handtype() == other.get_handtype()
            && self
                .cards
                .iter()
                .zip(other.cards.iter())
                .all(|(a, b)| a == b)
    }
}

pub fn process1(input: &str) -> u64 {
    let stime = std::time::Instant::now();

    let mut hands = input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();

            // Populate the cards
            let mut cards = [0u8; 5];
            it.next().unwrap().chars().enumerate().for_each(|(i, c)| {
                cards[i] = match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => c.to_digit(10).unwrap() as u8,
                };
            });

            let bid = it.next().unwrap().parse::<u32>().unwrap();
            Hand { cards, bid }
        })
        .collect::<Vec<Hand>>();

    hands.sort();

    let mut sol = 0;
    for (i, h) in hands.into_iter().enumerate() {
        sol += (h.bid as u64) * (i as u64 + 1);
    }

    println!("Day 7 - Part 1: {} [{:?}]", sol, stime.elapsed());
    sol
}

pub fn process2(input: &str) -> u64 {
    let stime = std::time::Instant::now();

    let mut hands = input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();

            // Populate the cards
            let mut cards = [0u8; 5];
            it.next().unwrap().chars().enumerate().for_each(|(i, c)| {
                cards[i] = match c {
                    'T' => 10,
                    'J' => 0, // Map jokers to 0.
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => c.to_digit(10).unwrap() as u8,
                };
            });

            let bid = it.next().unwrap().parse::<u32>().unwrap();
            Hand { cards, bid }
        })
        .collect::<Vec<Hand>>();

    hands.sort();
    // println!("{:?}", hands);

    let mut sol = 0;
    for (i, h) in hands.into_iter().enumerate() {
        sol += (h.bid as u64) * (i as u64 + 1);
    }

    println!("Day 7 - Part 2: {} [{:?}]", sol, stime.elapsed());
    sol
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process1(input), 6440)
    }

    #[test]
    fn test_process2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process2(input), 5905)
    }
}
