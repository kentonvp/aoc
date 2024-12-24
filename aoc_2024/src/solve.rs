use std::env;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn parse_args() -> (u16, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }
    (
        args[1].parse().unwrap(),
        format!("inputs/day{}.txt", args[1]),
    )
}

fn main() {
    let (day, file) = parse_args();
    let contents = fs::read_to_string(&file).unwrap();
    match day {
        1 => day1::solve(&contents),
        2 => day2::solve(&contents),
        3 => day3::solve(&contents),
        4 => day4::solve(&contents),
        5 => day5::solve(&contents),
        6 => day6::solve(&contents),
        _ => println!("Invalid day"),
    }
}
