use std::env::args;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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

        "3" => {
            let input = include_str!("../inputs/day03.txt");
            day3::part1(input);
            day3::part2(input);
        }
        "4" => {
            let input = include_str!("../inputs/day04.txt");
            day4::part1(input);
            day4::part2(input);
        }
        "5" => {
            let input = include_str!("../inputs/day05.txt");
            day5::part1(input);
            day5::part2(input);
        }
        _ => eprintln!("ERROR: Day not implemented"),
    }
}
