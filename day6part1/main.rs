use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Eq)]
struct Point {
    x: usize,
    y: usize
}

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

struct Position {
    point: Point,
    direction: Direction,
}

fn main() {
    let map: Vec<Vec<char>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut position = find_starting_position(&map).unwrap();
    let mut visited: HashSet<Point> = HashSet::from([position.point]);
    loop {
        let progressed = progress(&map, &mut position);
        if progressed.is_none() {
            break;
        }
        visited.insert(position.point);
    }
    println!("{:?}", visited.len());
}

fn find_starting_position(map: &Vec<Vec<char>>) -> Option<Position> {
    for (y, line) in map.iter().enumerate() {
        for (x, symbol) in line.iter().enumerate() {
            match symbol {
                '^' => return Some(Position{point: Point{x,y}, direction: Direction::UP}),
                '>' => return Some(Position{point: Point{x,y}, direction: Direction::RIGHT}),
                'v' => return Some(Position{point: Point{x,y}, direction: Direction::DOWN}),
                '<' => return Some(Position{point: Point{x,y}, direction: Direction::LEFT}),
                _ => continue
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
