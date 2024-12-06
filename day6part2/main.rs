use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
struct Point {
    x: usize,
    y: usize
}


#[derive(Clone)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}
impl Direction {
    pub fn turn_right(&mut self) -> Direction {
        match self {
            Direction::UP =>  Direction::RIGHT,
            Direction::RIGHT =>  Direction::DOWN,
            Direction::DOWN =>  Direction::LEFT,
            Direction::LEFT =>  Direction::UP,
        }
    }
}

#[derive(Clone)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
struct Position {
    point: Point,
    direction: Direction,
}

fn main() {
    let mut map: Vec<Vec<char>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let position = find_starting_position(&map).unwrap();
    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '#' || map[y][x] == '^' {
                // can't place an obstacle here
                continue
            }
            map[y][x] = '#';
            if will_be_stuck(&map, &position) {
                result += 1;
            }
            map[y][x] = '.';
        }
    }
    println!("{:?}", result);
}

fn will_be_stuck(map: &Vec<Vec<char>>, starting_position: &Position) -> bool {
    let mut position = starting_position.clone();
    let mut visited: HashSet<Position> = HashSet::from([position.clone()]);
    loop {
        let progressed = progress(&map, &mut position);
        if progressed.is_none() {
            return false;
        }
        if visited.contains(&position) {
            return true;
        }
        visited.insert(position.clone());
    }
}

fn find_starting_position(map: &Vec<Vec<char>>) -> Option<Position> {
    for (y, line) in map.iter().enumerate() {
        for (x, symbol) in line.iter().enumerate() {
            if symbol == &'^' {
                return Some(Position{point: Point{x,y}, direction: Direction::UP})
            }
        }
    }
    return None
}

fn progress(map: &Vec<Vec<char>>, position: &mut Position) -> Option<()> {
    let mut point = position.point.clone();
    match position.direction {
        Direction::LEFT => {
            point.x = point.x.checked_sub(1)?;
        },
        Direction::UP => {
            point.y = point.y.checked_sub(1)?;
        },
        Direction::RIGHT => {
            point.x += 1;
        },
        Direction::DOWN => {
            point.y += 1;
        },
    }
    if map.get(point.y)?.get(point.x)? == &'#' {
        position.direction = position.direction.turn_right();
    } else {
        position.point = point;
    }
    return Some(());
}
