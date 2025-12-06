struct RangePair(usize, usize);

impl RangePair {
    fn count(&self) -> usize {
        self.1 - self.0 + 1
    }
}

pub fn part1(input: &str) -> usize {
    // Parse ranges
    let mut breakline = 0;
    let mut ranges = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        // Empty lines breaks between range inputs and numbers to check
        if line.is_empty() {
            breakline = i;
            break;
        }

        let mut bounds = line.splitn(2, '-');
        let min = bounds.next().unwrap().parse::<usize>().unwrap();
        let max = bounds.next().unwrap().parse::<usize>().unwrap();
        ranges.push(RangePair(min, max));
    }

    // Check numbers against ranges
    let mut total = 0;
    for line in input.lines().skip(breakline + 1) {
        let line = line.trim();
        let num = line.parse::<usize>().unwrap();

        for pair in &ranges {
            if num >= pair.0 && num <= pair.1 {
                total += 1;
                break;
            }
        }
    }

    println!("Day 5, Part 1: {}", total);
    total
}

struct RangeSet {
    ranges: Vec<RangePair>,
}

impl RangeSet {
    fn new() -> Self {
        RangeSet { ranges: Vec::new() }
    }

    fn is_disjoint(&self, range_pair: &RangePair) -> bool {
        for rp in &self.ranges {
            if range_pair.0 <= rp.1 && range_pair.1 >= rp.0 {
                return false;
            }
        }
        true
    }

    fn add_range(&mut self, range_pair: RangePair) {
        // TODO: Check if new range overlaps with existing ranges and merge if necessary
        if self.ranges.is_empty() {
            self.ranges.push(range_pair);
            return;
        }

        if self.is_disjoint(&range_pair) {
            self.ranges.push(range_pair);
            return;
        }

        let mut overlapping_idx = None;
        for (i, rp) in self.ranges.iter_mut().enumerate() {
            if range_pair.0 <= rp.1 && range_pair.1 >= rp.0 {
                rp.0 = rp.0.min(range_pair.0);
                rp.1 = rp.1.max(range_pair.1);
                overlapping_idx = Some(i);
                break;
            }
        }

        let overlapping_idx = overlapping_idx.expect("will have value");
        let rp = self.ranges.remove(overlapping_idx);

        // merge rp and range_pair
        let merged = RangePair(rp.0.min(range_pair.0), rp.1.max(range_pair.1));
        self.add_range(merged);
    }

    fn count(&self) -> usize {
        let mut total = 0;
        for pair in &self.ranges {
            total += pair.count();
        }
        total
    }
}

pub fn part2(input: &str) -> usize {
    let mut ranges = RangeSet::new();
    for line in input.lines() {
        let line = line.trim();
        // Break at first empty line -- ignore the rest of the input
        if line.is_empty() {
            break;
        }

        let mut bounds = line.splitn(2, '-');
        let min = bounds.next().unwrap().parse::<usize>().unwrap();
        let max = bounds.next().unwrap().parse::<usize>().unwrap();

        ranges.add_range(RangePair(min, max));
    }

    println!("Day 5, Part 2: {}", ranges.count());
    ranges.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_range_set() {
        let mut range_set = RangeSet::new();
        range_set.add_range(RangePair(3, 5));
        assert_eq!(range_set.count(), 3);

        // disjoint add
        range_set.add_range(RangePair(10, 14));
        assert_eq!(range_set.count(), 8);

        range_set.add_range(RangePair(9, 12));
        assert_eq!(range_set.count(), 9);
    }

    #[test]
    fn test_part2() {
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;
        assert_eq!(part2(input), 14);
    }
}
