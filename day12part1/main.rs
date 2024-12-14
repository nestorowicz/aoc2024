use std::{collections::HashSet, io::{read_to_string, stdin}};

fn main() {
    let garden: Vec<Vec<char>> = read_to_string(stdin())
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let mut sum: u64 = 0;
    let mut mem: HashSet<(usize, usize)> = HashSet::new();
    for (y, line) in garden.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            sum += calculate_region_price(&garden, &mut mem, y, x)
        }
    }

    println!("{}", sum);
}

fn calculate_region_price(garden: &Vec<Vec<char>>, mem: &mut HashSet<(usize, usize)>, y: usize, x: usize) -> u64 {
    if let Some((area, perimeter)) = visit(garden, mem, y, x) {
        return area * perimeter;
    }
    return 0;
}

fn visit(garden: &Vec<Vec<char>>, mem: &mut HashSet<(usize, usize)>, y: usize, x: usize) -> Option<(u64, u64)> {
    if mem.contains(&(y, x)) {
        return None
    }
    mem.insert((y, x));

    let symbol = garden.get(y)?.get(x)?;
    let mut area = 1;
    let mut perimeter = 4;

    if y > 0 {
        if garden[y-1][x] == *symbol {
            perimeter -= 1;
            if let Some((area_up, perimeter_up)) = visit(garden, mem, y-1, x) {
                area += area_up;
                perimeter += perimeter_up;
            }
        }
    }
    if x > 0 {
        if garden[y][x-1] == *symbol {
            perimeter -= 1;
            if let Some((area_left, perimeter_left)) = visit(garden, mem, y, x-1) {
                area += area_left;
                perimeter += perimeter_left;
            }
        }
    }
    if y < garden.len()-1 {
        if garden[y+1][x] == *symbol {
            perimeter -= 1;
            if let Some((area_down, perimeter_down)) = visit(garden, mem, y+1, x) {
                area += area_down;
                perimeter += perimeter_down;
            }
        }
    }
    if x < garden.len()-1 {
        if garden[y][x+1] == *symbol {
            perimeter -= 1;
            if let Some((area_right, perimeter_right)) = visit(garden, mem, y, x+1) {
                area += area_right;
                perimeter += perimeter_right;
            }
        }
    }

    return Some((area, perimeter));
}

