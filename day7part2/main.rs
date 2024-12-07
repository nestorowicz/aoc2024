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

const BASE: u64 = 3;
const ADD: u64 = 0;
const MULTIPLY: u64 = 1;
const CONCAT: u64 = 2;

fn solve(equation: &Equation) -> Option<i64> {
    let numbers = &equation.numbers;
    let mut operators: u64 = 0;
    while get_operator(&operators, (numbers.len()-1) as u32) == 0 { 
        let mut sum: i64 = numbers[0];
        for i in 1..(numbers.len() as u32) {
            let next = numbers.get(i as usize)?;
            match get_operator(&operators, i-1) {
                ADD => sum += next,
                MULTIPLY => sum *= next,
                CONCAT => sum = concat(sum, *next),
                _ => panic!("what the")
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

fn get_operator(operators: &u64, position: u32) -> u64 {
    operators / BASE.pow(position) % BASE
}

fn concat(left: i64, right: i64) -> i64 {
    left * 10i64.pow(right.checked_ilog10().unwrap() + 1) + right
}

