use std::{collections::HashMap, io::{read_to_string, stdin}};

const ITERATIONS: u8 = 75;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Clone)]
struct Stone {
    value: u64,
    iteration: u8
}

fn main() {
    let mut queue: Vec<Stone> = read_to_string(stdin()).unwrap()
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .map(|n| Stone{value: n, iteration: 0})
        .rev()
        .collect();
    let mut cache: HashMap<Stone, u64> = HashMap::new();

    let mut sum: u64 = 0;
    while !queue.is_empty() {
        sum += solve(&queue.pop().unwrap().clone(), &mut cache);
    }


    println!("{}", sum);
}

fn cached_solve(stone: Stone, cache: &mut HashMap<Stone, u64>) -> u64 {
    if let Some(cached) = cache.get(&stone) {
        return *cached;
    }
    let sum = solve(&stone, cache);
    cache.insert(stone, sum);
    return sum;
}

fn solve(stone: &Stone, cache: &mut HashMap<Stone, u64>) -> u64 {
    let mut iteration = stone.iteration;
    if iteration == ITERATIONS {
        return 1
    }
    iteration += 1;

    let num = stone.value;
    if num == 0 {
        return cached_solve(Stone{value: 1, iteration}, cache);
    }

    let digits = num.ilog10() + 1;
    if digits % 2 == 0 {
        return cached_solve(Stone{value: left(num, digits), iteration}, cache) +
               cached_solve(Stone{value: right(num, digits), iteration}, cache);
    }

    return cached_solve(Stone{value: num*2024, iteration}, cache);
}

fn left(num: u64, digits: u32) -> u64 {
    return num / (10_u64.pow(digits/2))
}

fn right(num: u64, digits: u32) -> u64 {
    return num % (10_u64.pow(digits/2))
}

