pub fn process1(input: &str) -> u32 {
    let stime = std::time::Instant::now();
    println!("Day N - Part 1: {} [{:?}]", sum, stime.elapsed());
    todo!()
}

pub fn process2(input: &str) -> u32 {
    let stime = std::time::Instant::now();
    println!("Day N - Part 2: {} [{:?}]", sum, stime.elapsed());
    total()
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
