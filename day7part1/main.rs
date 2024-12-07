use std::fs::read_to_string;

struct Equation {
    sum: i64,
    numbers: Vec<i64>,
}

fn main() {
    let result: i64 = read_to_string("input.txt")
        .unwrap()
        .lines()
        .filter_map(|line| parse_equation(line))
        .filter_map(|equation| solve(&equation))
        .sum();

    println!("{:?}", result);
}

fn parse_equation(line: &str) -> Option<Equation> {
    let mut parts = line.split(": ");
    let sum: i64 = parts.next()?.parse().ok()?;
    let numbers: Vec<i64> = parts.next()?
        .split(" ")
        .filter_map(|input| input.parse::<i64>().ok())
        .collect();
    return Some(Equation{sum, numbers});
}

fn solve(equation: &Equation) -> Option<i64> {
    // bits in operators represent signs:
    // 0 -> +
    // 1 -> *
    let mut operators = 0;
    let numbers = &equation.numbers;
    while operators & (1 << numbers.len()-1) == 0 {
        let mut sum: i64 = numbers[0];
        for i in 1..numbers.len() {
            match operators & (1 << i-1) {
                0 => sum += numbers.get(i)?,
                _ => sum *= numbers.get(i)?
            }
            if sum > equation.sum {
                // no need to keep adding/multiplying if the sum is already too large
                break
            }
        }
        if sum == equation.sum {
            return Some(sum);
        }
        // add 1 to shift operators
        operators += 1;
    }

    return None;
}

