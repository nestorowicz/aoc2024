use std::{collections::HashMap, io::{read_to_string, stdin}};
use aoc::{Point, Map, Direction, Ray};

const START: char = 'S';
const END: char = 'E';
const WALL: char = '#';
const INITIAL_DIRECTION: Direction = Direction::Right;
const COST_MOVE: u64 = 1;
const COST_ROTATE: u64 = 1000;

struct Input {
    map: Map<char>,
    start: Point,
    end: Point
}

type Memory = HashMap<Point, u64>;

fn main() {
    let input = parse_input();
    aoc::print_char_map(&input.map);
    let min_cost: u64 = find_min_cost(&input);
    println!("{}", min_cost);
}

fn parse_input() -> Input {
    let map = Map{state:
        read_to_string(stdin()).unwrap()
            .lines()
            .map(|s| s.chars().collect())
            .collect()
    };
    let start = map.find_position(&START).unwrap();
    let end = map.find_position(&END).unwrap();
    return Input{map, start, end};
}

fn find_min_cost(input: &Input) -> u64 {
    let mut memory: HashMap<Point, u64> = HashMap::new();
    find_min_cost_node(input, &Ray::new(input.start, INITIAL_DIRECTION), &mut memory, 0);
    return *memory.get(&input.end).unwrap_or(&0);
}

fn find_min_cost_node(input: &Input, ray: &Ray, memory: &mut Memory, cost: u64) {
    let Some(symbol) = input.map.peek(&ray.point) else { return };
    if *symbol == WALL { return }

    if memory.contains_key(&ray.point) && memory.get(&ray.point).unwrap() < &cost { return }
    memory.insert(ray.point, cost);

    if *symbol == END { return };

    if let Some(ray) = ray.move_by_one() {
        find_min_cost_node(input, &ray, memory, cost + COST_MOVE);
    }
    if let Some(ray) = ray.rotate_90_clockwise().move_by_one() {
        find_min_cost_node(input, &ray, memory, cost + COST_ROTATE + COST_MOVE);
    }
    if let Some(ray) = ray.rotate_90_counter_clockwise().move_by_one() {
        find_min_cost_node(input, &ray, memory, cost + COST_ROTATE + COST_MOVE);
    }
}

