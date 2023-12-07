pub fn process1(input: &str) -> u64 {
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
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<u64>>();

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
            .map(str::parse::<u64>)
            .map(Result::unwrap);
        let dest = mapping.next().expect("Destination");
        let source = mapping.next().expect("Source");
        let offset = mapping.next().expect("Offset");

        seeds = seeds
            .iter()
            .enumerate()
            .map(|(i, &s)| {
                if map_done[i] {
                    s
                } else if (source <= s) && (s < (source + offset)) {
                    // Mark as done.
                    map_done[i] = true;
                    let delta = s - source;
                    dest + delta
                } else {
                    s
                }
            })
            .collect::<Vec<u64>>();
    }

    // Find minimum.
    let sol = seeds.iter().min().unwrap();

    println!("Day 5 - Part 1: {} [{:?}]", sol, stime.elapsed());
    *sol
}

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone)]
struct Mapping {
    dest: Range,
    source: Range,
}

impl Mapping {
    fn new(dest: u64, source: u64, offset: u64) -> Self {
        Self {
            dest: Range::new(dest, dest + offset),
            source: Range::new(source, source + offset),
        }
    }

    fn apply(&self, range: &Range) -> [Option<Vec<Range>>; 2] {
        // Result Element 1 is the leftover Ranges in the source space. Element 2 are Ranges in the
        // destination space.
        let mut result = [None, None];

        if range.end <= self.source.start || self.source.end <= range.start {
            // No overlap.
            result[0] = Some(vec![range.clone()]);
        } else if range.start < self.source.start
            && self.source.start < range.end
            && range.end <= self.source.end
        {
            // Partial overlap: mapped right part.
            result[0] = Some(vec![Range::new(range.start, self.source.start)]);
            result[1] = Some(vec![Range::new(
                self.dest.start,
                self.dest.start + range.end - self.source.start,
            )]);
        } else if self.source.start <= range.start
            && range.start < self.source.end
            && self.source.end < range.end
        {
            // Partial overlap: mapped left part.
            result[0] = Some(vec![Range::new(self.source.end, range.end)]);
            result[1] = Some(vec![Range::new(
                self.dest.start + range.start - self.source.start,
                self.dest.end
            )]);
        } else if range.start < self.source.start && self.source.end < range.end {
            // Range larger than source.
            result[0] = Some(vec![
                Range::new(range.start, self.source.start),
                Range::new(self.source.end, range.end),
            ]);
            result[1] = Some(vec![self.dest.clone()]);
        } else if self.source.start <= range.start && range.end <= self.source.end {
            // Range smaller than source.
            result[1] = Some(vec![Range::new(
                self.dest.start + range.start - self.source.start,
                self.dest.start + range.end - self.source.start,
            )]);
        } else {
            panic!("Unhandled case: {:?} {:?}", self, range);
        }
        result
    }
}

pub fn process2(input: &str) -> u64 {
    let stime = std::time::Instant::now();
    let mut it = input.lines();

    // Parse seed ranges.
    let seed_ranges = it
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(str::trim)
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<u64>>();

    let mut seeds = Vec::new();
    for i in (0..seed_ranges.len()).step_by(2) {
        let s = seed_ranges[i];
        seeds.push(Range::new(s, s + seed_ranges[i + 1] - 1));
    }

    // Skip empty line.
    let _ = it.next();

    // Parse Mappings.
    let mut mappings = Vec::new();
    let mut curr_map = Vec::new();
    for l in it {
        if l.is_empty() {
            // Mapping finished.
            mappings.push(curr_map.clone());
            curr_map = Vec::new();
            continue;
        }

        if l.starts_with(char::is_alphabetic) {
            // New mapping.
            continue;
        }

        let mut mapping = l
            .split_whitespace()
            .map(str::parse::<u64>)
            .map(Result::unwrap);
        let dest = mapping.next().expect("Destination");
        let source = mapping.next().expect("Source");
        let offset = mapping.next().expect("Offset");
        curr_map.push(Mapping::new(dest, source, offset));
    }
    // Add last mapping.
    mappings.push(curr_map.clone());

    for (_i, map_group) in mappings.iter().enumerate() {
        let mut finished_ranges = Vec::new();
        for m in map_group.iter() {
            let mut unfinished_ranges = Vec::new();
            for r in seeds.iter() {
                let mut res = m.apply(r);
                if let Some(mut unmapped) = res[0].take() {
                    unfinished_ranges.append(&mut unmapped);
                }
                if let Some(mut mapped) = res[1].take() {
                    finished_ranges.append(&mut mapped);
                }
            }

            seeds.clear();
            seeds.append(&mut unfinished_ranges.clone());
        }
        seeds.append(&mut finished_ranges.clone());
    }

    // Find minimum.
    let sol = seeds.into_iter().map(|r| r.start).min().unwrap();

    println!("Day 5 - Part 2: {} [{:?}]", sol, stime.elapsed());
    sol
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

    #[test]
    fn test_process2_real() {
        let input = std::fs::read_to_string("inputs/day5.txt").unwrap();
        assert_eq!(process2(&input), 84206669)
    }
}
