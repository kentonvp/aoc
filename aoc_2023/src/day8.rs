use std::collections::HashMap;
use num::integer::lcm;

#[derive(Clone, Debug)]
struct Node {
    value: String,
    pair: [String; 2],
}

fn binary_search(mapping: &Vec<Node>, key: String) -> &Node {
    let mut left = 0usize;
    let mut mid = mapping.len() / 2;
    let mut right = mapping.len();

    let mut curr = mapping[mid].value.as_str();
    while key.as_str() != curr {
        if key.as_str() < curr {
            // Search left.
            right = mid;
            mid = (left + mid) / 2;
        } else {
            // Search right.
            left = mid;
            mid = (right + mid) / 2;
        }
        curr = mapping[mid].value.as_str();
    }
    return &mapping[mid];
}

pub fn process1(input: &str) -> u32 {
    let stime = std::time::Instant::now();

    // Map directions to 0 (left) and 1 (right) indexes.
    let mut directions = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid right/left"),
        })
        .cycle();

    let mut mappings = input
        .lines()
        .skip(2)
        .map(|l| {
            let mut it = l.split_whitespace();
            let value = it.next().unwrap();
            it.next();

            // Remove the first char '(' and the ','.
            let mut left = it.next().unwrap().to_string();
            left = left[1..left.len() - 1].to_string();

            // Remove the ')' at the end.
            let mut right = it.next().unwrap().to_string();
            right = right[..right.len() - 1].to_string();

            Node {
                value: value.to_string(),
                pair: [left, right],
            }
        })
        .collect::<Vec<Node>>();
    mappings.sort_by_key(|n| n.value.clone());

    let mut steps = 0;
    let mut curr = binary_search(&mappings, "AAA".to_string());
    while !curr.value.eq("ZZZ") {
        steps += 1;

        let dir = directions.next().unwrap();
        let find = &curr.pair[dir];
        curr = binary_search(&mappings, find.to_string());
    }

    println!("Day 8 - Part 1: {} [{:?}]", steps, stime.elapsed());
    steps
}

pub fn process2(input: &str) -> u64 {
    let stime = std::time::Instant::now();

    // Map directions to 0 (left) and 1 (right) indexes.
    let mut directions = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid right/left"),
        })
        .cycle();

    let mut mappings = input
        .lines()
        .skip(2)
        .map(|l| {
            let mut it = l.split_whitespace();
            let value = it.next().unwrap();
            it.next();

            // Remove the first char '(' and the ','.
            let mut left = it.next().unwrap().to_string();
            left = left[1..left.len() - 1].to_string();

            // Remove the ')' at the end.
            let mut right = it.next().unwrap().to_string();
            right = right[..right.len() - 1].to_string();

            Node {
                value: value.to_string(),
                pair: [left, right],
            }
        })
        .collect::<Vec<Node>>();
    mappings.sort_by_key(|n| n.value.clone());
    
    // Cache the binary search results.
    let mut cache = HashMap::new();

    let starts = mappings
        .iter()
        .filter(|n| n.value.ends_with("A"))
        .collect::<Vec<&Node>>();

    let mut curr = starts.clone();

    let mut cycles = vec![0usize; curr.len()];
    let mut steps = 0;
    while !(cycles.iter().all(|&n| n > 0))
    {
        steps += 1;
        let dir = directions.next().unwrap();

        curr = curr
            .iter()
            .map(|&n| {
                let find = &n.pair[dir];
                if let Some(found) = cache.get(find) {
                    found
                } else {
                    let found = binary_search(&mappings, find.to_string());
                    cache.insert(find, found);
                    found
                }
            })
            .collect::<Vec<&Node>>();

        curr.iter().enumerate().for_each(|(i, &n)| {
            if n.value.ends_with("Z") && cycles[i] == 0 {
                cycles[i] = steps;
            }
        });
    }

    // Find the LCM of all the cycles.
    let mut sol = cycles[0] as u64;
    for i in 1..cycles.len() {
        sol = lcm(sol, cycles[i] as u64);
    }

    println!("Day 8 - Part 2: {} [{:?}]", sol, stime.elapsed());
    sol
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1a() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process1(input), 2)
    }

    #[test]
    fn test_process1b() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process1(input), 6)
    }

    #[test]
    fn test_process2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(process2(input), 6)
    }
}
