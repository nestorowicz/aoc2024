use std::io::{read_to_string, stdin};
use aoc::{Point, Map, Direction};

const ROBOT: char = '@';
const BOX: char = 'O';
const NOTHING: char = '.';

struct State {
    map: Map<char>,
    moves: Vec<char>
}

fn main() {
    let mut state = parse_input();
    let mut robot = state.map.iter_points()
        .filter_map(|p| Some((p, state.map.peek(&p)?)))
        .find(|(_, c)| *c == &ROBOT)
        .map(|(p, _)| p)
        .unwrap();

    aoc::print_char_map(&state.map);
    
    state.moves.iter().for_each(|m| apply_move(&mut state.map, &mut robot, *m));
    let checksum: usize = state.map.iter_points()
        .filter_map(|p| Some((p, state.map.peek(&p)?)))
        .filter(|(_, c)| **c == BOX)
        .map(|(p, _)| p.y*100+p.x)
        .sum();
    println!("{}", checksum);
}

fn parse_input() -> State {
    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();
    let map: Map<char> = aoc::parse_map(&mut lines);
    let moves: Vec<char> = lines.flat_map(|line| line.chars().collect::<Vec<char>>()).collect();
    return State{map, moves};
}

fn apply_move(map: &mut Map<char>, robot: &mut Point, m: char) {
    let dir = match m {
        '<' => Some(Direction::Left),
        '^' => Some(Direction::Up),
        '>' => Some(Direction::Right),
        'v' => Some(Direction::Down),
        _ => None
    };
    let Some(dir) = dir else { return; };

    let p = move_dir(map, robot.clone(), &dir);
    robot.x = p.x;
    robot.y = p.y;
}

fn move_dir(map: &mut Map<char>, point: Point, dir: &Direction) -> Point {
    let Some(c) = map.peek(&point) else { return point };
    let c = *c;
    if c != ROBOT && c != BOX { return point; }

    let Some(next) = map.move_dir(&point, dir) else { return point; };
    move_dir(map, next, dir);
    let Some(l) = map.peek(&next) else { return point };
    if *l != NOTHING { return point };

    map.state[next.y][next.x] = c;
    map.state[point.y][point.x] = NOTHING;
    return next;
}
