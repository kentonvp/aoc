mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    dotenv::dotenv().ok();
    let session = std::env::var("AOC_SESSION_TOKEN").expect("AOC_SESSION_TOKEN not set");

    let args = std::env::args().collect::<Vec<String>>();
    let day = args
        .get(1)
        .expect("INVALID ARGUMENTS::requires the challenge day")
        .parse::<u8>()
        .unwrap();
    let fname = download_input(day, &session);

    let input = std::fs::read_to_string(fname).unwrap();

    match day {
        1 => {
            day1::process1(&input);
            day1::process2(&input);
        }
        2 => {
            day2::process1(&input);
            day2::process2(&input);
        }
        3 => {
            day3::process1(&input);
            day3::process2(&input);
        }
        4 => {
            day4::process1(&input);
            day4::process2(&input);
        }
        5 => {
            day5::process1(&input);
            day5::process2(&input);
        }
        6 => {
            day6::process1(&input);
            day6::process2(&input);
        }
        7 => {
            day7::process1(&input);
            day7::process2(&input);
        }
        _ => panic!("Day {} not implemented", day),
    }
}

fn download_input(day: u8, session: &str) -> String {
    let input_file = format!("inputs/day{}.txt", day);
    if std::path::Path::new(&input_file).exists() {
        println!("{} already exists", input_file);
        return input_file;
    }

    let url = format!("https://adventofcode.com/2023/day/{}/input", day);

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get(url)
        .header("Cookie", format!("session={}", session))
        .send()
        .map_err(|e| {
            eprintln!("ERROR::{}", e);
            panic!("Error making request to download input file -- day={day}")
        })
        .unwrap();

    if resp.status() != 200 {
        eprintln!("{:?}", resp);
        panic!("Error downloading input file -- day={day}")
    }

    let body = resp.text().unwrap();
    std::fs::write(&input_file, body).unwrap();

    println!("{}", input_file);
    input_file
}
