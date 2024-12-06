use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct Update {
    values: HashMap<i32, usize>,
    middle: i32,
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let rules: Vec<(i32, i32)> = input
        .lines()
        .map_while(|line| parse_rule(line))
        .collect();

    let result = input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
        .map(|line| parse_updates(line))
        .filter(|update| validate_update(update, &rules))
        .fold(0, |acc, element| acc+element.middle);

    println!("{:?}", result);
}

// parse_rule parses "num|num" strings into tuples
fn parse_rule(line: &str) -> Option<(i32, i32)> {
    let mut parts = line.split("|");
    let first: i32 = parts.next()?.parse::<i32>().ok()?;
    let second: i32 = parts.next()?.parse::<i32>().ok()?;
    return Some((first, second));
}

// parse_updated parses a comma-separated string of numbers into a HashMap of values to their
// indices
fn parse_updates(line: &str) -> Update {
    let vals: Vec<i32> = line
        .split(",")
        .map(|part| part.parse::<i32>().unwrap())
        .collect();
    let mid = vals[vals.len()/2];
    let vals_index: HashMap<i32, usize> =
        vals.iter().enumerate()
        .map(|(i, num)| (*num, i))
        .collect::<HashMap<i32, usize>>();
    return Update{values: vals_index, middle: mid};
}

// validate_update returns true if the update is valid according to provided rules
fn validate_update(update: &Update, rules: &Vec<(i32, i32)>) -> bool {
    return rules.iter()
        .filter_map(|(before, after)| validate_rule(update, *before, *after))
        .all(|result| result);
}
fn validate_rule(update: &Update, before: i32, after: i32) -> Option<bool> {
    let left_idx = update.values.get(&before)?;
    let right_idx = update.values.get(&after)?;
    if left_idx > right_idx {
        return Some(false);
    }
    return Some(true);
}

