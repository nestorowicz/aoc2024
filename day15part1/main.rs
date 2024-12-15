use std::io::{read_to_string, stdin};
use aoc::{Point, Map};

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
    let p = match m {
        '<' => move_left(map, robot.clone()),
        '^' => move_up(map, robot.clone()),
        '>' => move_right(map, robot.clone()),
        'v' => move_down(map, robot.clone()),
        _ => robot.clone() 
    };
    robot.x = p.x;
    robot.y = p.y;
}

fn move_left(map: &mut Map<char>, point: Point) -> Point {
    let Some(c) = map.peek(&point) else { return point };
    let c = *c;
    if c != ROBOT && c != BOX { return point; }

    let Some(left) = map.move_left(&point) else { return point; };
    move_left(map, left);
    let Some(l) = map.peek(&left) else { return point };
    if *l != NOTHING { return point };

    map.state[left.y][left.x] = c;
    map.state[point.y][point.x] = NOTHING;
    return left;
}
fn move_up(map: &mut Map<char>, point: Point) -> Point {
    let Some(c) = map.peek(&point) else { return point; };
    let c = *c;
    if c != ROBOT && c != BOX { return point; }

    let Some(up) = map.move_up(&point) else { return point; };
    move_up(map, up);
    let Some(l) = map.peek(&up) else { return point; };
    if *l != NOTHING { return point; };

    map.state[up.y][up.x] = c;
    map.state[point.y][point.x] = NOTHING;
    return up;
}
fn move_right(map: &mut Map<char>, point: Point) -> Point {
    let Some(c) = map.peek(&point) else { return point; };
    let c = *c;
    if c != ROBOT && c != BOX { return point; }

    let Some(right) = map.move_right(&point) else { return point; };
    move_right(map, right);
    let Some(l) = map.peek(&right) else { return point; };
    if *l != NOTHING { return point; };

    map.state[right.y][right.x] = c;
    map.state[point.y][point.x] = NOTHING;
    return right
}
fn move_down(map: &mut Map<char>, point: Point) -> Point {
    let Some(c) = map.peek(&point) else { return point; };
    let c = *c;
    if c != ROBOT && c != BOX { return point; }

    let Some(down) = map.move_down(&point) else { return point; };
    move_down(map, down);
    let Some(l) = map.peek(&down) else { return point; };
    if *l != NOTHING { return point; };

    map.state[down.y][down.x] = c;
    map.state[point.y][point.x] = NOTHING;
    return down;
}

