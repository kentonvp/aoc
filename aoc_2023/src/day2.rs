pub fn process1(input: &str) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let mut sum: u32 = 0;

    for line in input.lines() {
        let eidx = line.find(':').unwrap();
        let id = &line[5..eidx].parse::<u32>().unwrap();

        let mut success = true;

        let games = &line[eidx + 1..]
            .split(';')
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        for game in games {
            let selections = game.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
            for op in selections {
                let mut it = op.split_whitespace();
                let val = it.next().unwrap().parse::<u32>().unwrap();
                let color = it.next().unwrap();

                match color {
                    "red" => {
                        if val > MAX_RED {
                            success = false;
                            break;
                        }
                    }
                    "green" => {
                        if val > MAX_GREEN {
                            success = false;
                            break;
                        }
                    }
                    "blue" => {
                        if val > MAX_BLUE {
                            success = false;
                            break;
                        }
                    }
                    _ => panic!("Invalid color"),
                }
            }
            if !success {
                break;
            }
        }

        if success {
            sum += id;
        }
    }

    println!("{sum}");
    return sum;
}

pub fn process2(input: &str) -> u32 {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let eidx = line.find(':').unwrap();

        let games = &line[eidx + 1..]
            .split(';')
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        let mut max_red: u32 = 0;
        let mut max_green: u32 = 0;
        let mut max_blue: u32 = 0;
        for game in games {
            let selections = game.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
            for op in selections {
                let mut it = op.split_whitespace();
                let val = it.next().unwrap().parse::<u32>().unwrap();
                let color = it.next().unwrap();

                match color {
                    "red" => {
                        if val > max_red {
                            max_red = val;
                        }
                    }
                    "green" => {
                        if val > max_green {
                            max_green = val;
                        }
                    }
                    "blue" => {
                        if val > max_blue {
                            max_blue = val
                        }
                    }
                    _ => panic!("Invalid color"),
                }
            }
        }

        let power = max_red * max_green * max_blue;
        sum += power;
    }

    println!("{sum}");
    return sum;
}

// Test
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_process1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(process1(input), 8)
    }

    #[test]
    fn test_process2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(process2(input), 2286)
    }
}
