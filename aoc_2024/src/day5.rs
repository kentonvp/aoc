// Day 5 //////////////////////////////////////////////////////////////////////
use std::collections::{HashMap, HashSet};

fn is_valid_print_order(printer: &HashMap<u32, HashSet<u32>>, print_order: &[u32]) -> bool {
    for (i, e) in print_order.iter().enumerate() {
        let (_, post_e) = print_order.split_at(i + 1);

        for p in post_e {
            if !printer.get(e).unwrap().contains(p) {
                return false;
            }
        }
    }
    true
}

fn part1(contents: &str) -> u32 {
    // Track the next elements
    let mut printer = HashMap::new();

    let mut line_iter = contents.lines();
    loop {
        let l = line_iter.next().expect("Invalid input.");
        if l.is_empty() {
            break;
        }

        let mut it = l.split("|");
        let a: u32 = it.next().unwrap().parse().unwrap();
        let b: u32 = it.next().unwrap().parse().unwrap();

        printer.entry(a).or_insert(HashSet::new()).insert(b);
        printer.entry(b).or_insert(HashSet::new());
    }

    let mut middle_sum = 0;
    for l in line_iter {
        let print_order = l
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if is_valid_print_order(&printer, &print_order) {
            middle_sum += print_order[print_order.len() / 2];
        }
    }

    middle_sum
}

fn move_elem(ordering: &mut Vec<u32>, s: usize, d: usize) {
    // move the element from s to d shifting the rest of the elements
    let e = ordering.remove(s);
    ordering.insert(d, e);
}

fn find_print_order_err(
    printer: &HashMap<u32, HashSet<u32>>,
    print_order: &[u32],
) -> Option<(usize, usize)> {
    for (i, e) in print_order.iter().enumerate() {
        let (_, post_e) = print_order.split_at(i + 1);

        for (j, p) in post_e.iter().enumerate() {
            if !printer.get(e).unwrap().contains(p) {
                return Some((i, i + j + 1));
            }
        }
    }
    None
}

fn correct_print_order(printer: &HashMap<u32, HashSet<u32>>, print_order: &mut Vec<u32>) {
    // while the print order is not valid
    // find the first element that is not valid
    // use move_elem to move it to the correct position
    // repeat until the print order is valid
    while let Some((i, j)) = find_print_order_err(printer, print_order) {
        move_elem(print_order, i, j);
    }
}

fn part2(contents: &str) -> u32 {
    let mut printer = HashMap::new();

    let mut middle_sum = 0;

    for l in contents.lines() {
        if l.contains("|") {
            let mut it = l.split("|");
            let a: u32 = it.next().unwrap().parse().unwrap();
            let b: u32 = it.next().unwrap().parse().unwrap();

            printer.entry(a).or_insert(HashSet::new()).insert(b);
            printer.entry(b).or_insert(HashSet::new());
        } else if l.contains(",") {
            let mut print_order = l
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            if !is_valid_print_order(&printer, &print_order) {
                correct_print_order(&printer, &mut print_order);
                middle_sum += print_order[print_order.len() / 2];
            }
        }
    }

    middle_sum
}

pub fn solve(contents: &str) {
    let time = std::time::Instant::now();
    let res = part1(contents);
    println!("Part1: {} ({:.4}s)", res, time.elapsed().as_secs_f64());

    let time = std::time::Instant::now();
    let res = part2(contents);
    println!("Part2: {} ({:.4}s)", res, time.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part1(contents), 143);
    }

    #[test]
    fn test_part2() {
        let contents = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part2(contents), 123);
    }
}
