fn is_operation_line(line: &str) -> bool {
    line.contains('+') || line.contains('*')
}

pub fn part1(input: &str) -> usize {
    let mut rows = Vec::new();
    let mut opline = None;
    for line in input.lines() {
        if is_operation_line(line) {
            opline = Some(line);
            break;
        }

        let data = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        rows.push(data);
    }

    let opline = opline.expect("will be operation line");
    let mut total = 0;
    for (i, op) in opline.split_whitespace().enumerate() {
        match op {
            "+" => {
                for row in &rows {
                    total += row[i];
                }
            }
            "*" => {
                let mut prod = 1;
                for row in &rows {
                    prod *= row[i];
                }

                total += prod;
            }
            _ => {}
        }
    }

    println!("Day 6, Part 1: {}", total);
    total
}

pub fn part2(input: &str) -> usize {
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut data = line.chars().collect::<Vec<char>>();
        data.reverse();
        rows.push(data);
    }

    let mut total = 0;
    let opline = rows.pop().expect("will be operation line");
    let mut operands = Vec::new();
    for (i, &op) in opline.iter().enumerate() {
        let mut operand = String::new();
        for row in &rows {
            if row[i].is_numeric() {
                operand.push(row[i]);
            }
        }

        // Empty column -> move to next equation!
        if operand.is_empty() {
            continue;
        }

        operands.push(operand.parse::<usize>().unwrap());

        if op == '+' {
            for operand in &operands {
                total += operand
            }
            operands.clear();
        } else if op == '*' {
            let mut prod = 1;
            for operand in &operands {
                prod *= operand;
            }
            total += prod;
            operands.clear();
        }
    }

    println!("Day 6, Part 2: {}", total);
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;
        assert_eq!(part1(input), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;
        assert_eq!(part2(input), 3263827);
    }
}
