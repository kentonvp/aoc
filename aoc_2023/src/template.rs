pub fn process1(input: &str) -> u32 {
    let stime = std::time::Instant::now();
    let sol = 0;
    println!("Day N - Part 1: {} [{:?}]", sol, stime.elapsed());
    sol
}

pub fn process2(input: &str) -> u32 {
    let stime = std::time::Instant::now();
    let sol = 0;
    println!("Day N - Part 2: {} [{:?}]", sol, stime.elapsed());
    sol
}

// Test
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_process1() {
        let input = "";
        assert_eq!(process1(input), 0)
    }

    #[test]
    fn test_process2() {
        let input = "";
        assert_eq!(process2(input), 0)
    }
}
