pub fn process1(input: &str) -> u32 {
    let stime = std::time::Instant::now();
    let mut it = input.lines();

    // Parse seed input
    let mut seeds = it
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(str::trim)
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect::<Vec<u32>>();

    // Skip empty line
    let _ = it.next();

    let mut map_done = vec![false; seeds.len()];
    let mut is_map_done = false;
    for l in it {
        if l.is_empty() {
            // Mapping finished.
            map_done = vec![false; seeds.len()];
            is_map_done = false;
            continue;
        }

        if l.starts_with(char::is_alphabetic) {
            // New mapping.
            continue;
        }

        // Short-circuit if all entries are already found
        if is_map_done || map_done.iter().all(|&b| b) {
            is_map_done = true;
            continue;
        }

        let mut mapping = l
            .split_whitespace()
            .map(str::parse::<u32>)
            .map(Result::unwrap);
        let dest = mapping.next().expect("Destination");
        let source = mapping.next().expect("Source");
        let offset = mapping.next().expect("Offset");

        seeds = seeds
            .iter()
            .enumerate()
            .map(|(i, s)| {
                if map_done[i] {
                    *s
                } else if (source <= *s) && (*s < (source + offset)) {
                    // Mark as done.
                    map_done[i] = true;
                    let delta = *s - source;
                    dest + delta
                } else {
                    *s
                }
            })
            .collect::<Vec<u32>>();
    }

    // Find minimum.
    let sol = seeds.iter().min().unwrap();

    println!("Day 5 - Part 1: {} [{:?}]", sol, stime.elapsed());
    *sol
}

pub fn process2(input: &str) -> u32 {
    let stime = std::time::Instant::now();
    let mut it = input.lines();

    // Parse seed input
    let seed_ranges = it
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(str::trim)
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect::<Vec<u32>>();

    // Skip empty line.
    let _ = it.next();

    let mut seeds = Vec::new();
    for i in (0..seed_ranges.len()).step_by(2) {
        for j in 0..seed_ranges[i+1] {
            seeds.push(seed_ranges[i] + j);
        }
    }

    let total_seeds = seeds.len();
    let mut map_done = vec![false; total_seeds];
    let mut is_map_done = false;
    let mut map_num = 0;
    for l in it {
        if l.is_empty() {
            println!("{} map done [took {:?}]", map_num, stime.elapsed());
            // Mapping finished.
            map_done.fill(false);
            is_map_done = false;
            continue;
        }

        if l.starts_with(char::is_alphabetic) {
            map_num += 1;
            // New mapping.
            continue;
        }

        // Short-circuit if all entries are already found
        if is_map_done || map_done.iter().all(|&b| b) {
            is_map_done = true;
            continue;
        }

        let mut mapping = l
            .split_whitespace()
            .map(str::parse::<u32>)
            .map(Result::unwrap);
        let dest = mapping.next().expect("Destination");
        let source = mapping.next().expect("Source");
        let offset = mapping.next().expect("Offset");

        seeds = seeds
            .iter().enumerate()
            .map(|(i, &s)| {
                if map_done[i] {
                    // Already found mapping. Skip.
                    s
                } else if (source <= s) && (s < (source + offset)) {
                    // Mark as done.
                    map_done[i] = true;
                    let delta = s - source;
                    dest + delta
                } else {
                    // No mapping found and not done. Keep original.
                    s
                }
            })
            .collect::<Vec<u32>>();
    }

    // Find minimum.
    let sol = seeds.iter().min().unwrap();

    println!("Day 5 - Part 2: {} [{:?}]", sol, stime.elapsed());
    *sol
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(process1(input), 35)
    }

    #[test]
    fn test_process2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(process2(input), 46)
    }
}
