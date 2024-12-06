use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct Update {
    values: HashMap<i32, usize>,
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
        .filter(|update| !validate_update(update, &rules))
        .map(|update| fix_update(&update, &rules))
        .map(|update| find_mid(&update))
        .fold(0, |acc, element| acc+element);

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
    let vals: HashMap<i32, usize> = line
        .split(",")
        .map(|part| part.parse::<i32>().unwrap())
        .enumerate()
        .map(|(i, num)| (num, i))
        .collect();
    return Update{values: vals};
}

// validate_update returns true if the update is valid according to provided rules
fn validate_update(update: &Update, rules: &Vec<(i32, i32)>) -> bool {
    for (left, right) in rules {
        if !update.values.contains_key(left) || !update.values.contains_key(right) {
            continue
        }
        if update.values[left] > update.values[right] {
            return false
        }
    }
    return true
}
fn fix_update(update: &Update, rules: &Vec<(i32, i32)>) -> Update {
    let mut fixed = update.values.clone();
    let mut is_fixed = false;
    while !is_fixed {
        is_fixed = true;
        for (left, right) in rules {
            if !fixed.contains_key(left) || !fixed.contains_key(right) {
                continue
            }
            let left_idx = fixed[left];
            let right_idx = fixed[right];
            if left_idx > right_idx {
                fixed.insert(*left, right_idx);
                fixed.insert(*right, left_idx);
                is_fixed = false;
            }
        }
    }
    return Update{values: fixed}
}
fn find_mid(update: &Update) -> i32 {
    let mut values: Vec<(i32, usize)> = update.values.iter()
        .map(|(key, val)| (*key, *val))
        .collect();
    values.sort_by(|(_, v1), (_, v2)| v1.cmp(v2));
    let values_sorted: Vec<i32> = values.iter().map(|(k, _)| *k).collect();
    return values_sorted[values_sorted.len()/2];
}
