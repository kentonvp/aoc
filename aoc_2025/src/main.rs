use std::env::args;

mod day1;
mod day2;

fn main() {
    let args = args().collect::<Vec<String>>();

    assert!(args.len() == 2, "This program takes one argument, the day");

    match args.last().unwrap().as_str() {
        "1" => {
            let input = include_str!("../inputs/day01.txt");
            day1::part1(input);
            day1::part2(input);
        }
        "2" => {
            let input = include_str!("../inputs/day02.txt");
            day2::part1(input);
            day2::part2(input);
        }
        _ => eprintln!("ERROR: Day not implemented"),
    }
}
