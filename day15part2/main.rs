use std::io::{read_to_string, stdin};
use aoc::{Point, Map, Direction};

const WALL: char = '#';
const ROBOT: char = '@';
const BOX_INPUT: char = 'O';
const BOX_LEFT: char = '[';
const BOX_RIGHT : char = ']';
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

    state.moves.iter().for_each(|m| apply_move(&mut state.map, &mut robot, *m));

    aoc::print_char_map(&state.map);

    let checksum: usize = state.map.iter_points()
        .filter_map(|p| Some((p, state.map.peek(&p)?)))
        .filter(|(_, c)| **c == BOX_LEFT)
        .map(|(p, _)| p.y*100+p.x)
        .sum();

    println!("{}", checksum);
}

fn parse_input() -> State {
    let input = read_to_string(stdin()).unwrap();
    let lines = &mut input.lines();
    let state: Vec<Vec<char>> = lines
        .take_while(|l| !l.is_empty())
        .map(|s| s.chars().flat_map(widen_input).collect())
        .collect();
    let map = Map{state};
    let moves: Vec<char> = lines.flat_map(|line| line.chars().collect::<Vec<char>>()).collect();
    return State{map, moves};
}

fn widen_input(c: char) -> Vec<char> {
    match c {
        WALL => vec![WALL, WALL],
        BOX_INPUT => vec![BOX_LEFT, BOX_RIGHT],
        NOTHING => vec![NOTHING, NOTHING],
        ROBOT => vec![ROBOT, NOTHING],
        _ => vec![]
    }
}

fn apply_move(map: &mut Map<char>, robot: &mut Point, m: char) {
    let Some(dir) = map_dir(m) else { return };
    let p = move_dir(map, robot.clone(), &dir);
    robot.x = p.x;
    robot.y = p.y;
}

fn map_dir(input: char) -> Option<Direction> {
    match input {
        '<' => Some(Direction::Left),
        '^' => Some(Direction::Up),
        '>' => Some(Direction::Right),
        'v' => Some(Direction::Down),
        _ => None
    }
}

fn move_dir(map: &mut Map<char>, point: Point, dir: &Direction) -> Point {
    let Some(c) = map.peek(&point).map(|c| *c) else { return point };
    if c != ROBOT && c != BOX_LEFT && c != BOX_RIGHT { return point; }
    if !can_move_dir(map, &point, dir) { return point; }
    let next = do_move(map, &point, dir);

    if (c == BOX_LEFT || c == BOX_RIGHT) && (*dir == Direction::Up || *dir == Direction:: Down) {
        let other = if c == BOX_LEFT { &Direction::Right } else { &Direction::Left };
        let other = map.move_dir(&point, other).unwrap();
        do_move(map, &other, dir);
    }

    return next;
}

fn do_move(map: &mut Map<char>, point: &Point, dir: &Direction) -> Point {
    let next = map.move_dir(point, dir).unwrap();
    move_dir(map, next, dir);
    map.state[next.y][next.x] = *map.peek(point).unwrap();
    map.state[point.y][point.x] = NOTHING;
    return next;
}

fn can_move_dir(map: &mut Map<char>, point: &Point, dir: &Direction) -> bool {
    let Some(c) = map.peek(&point).map(|c| *c) else { return false };
    if c == NOTHING {
        return true;
    }
    if c == ROBOT {
        let Some(next) = map.move_dir(point, dir) else { return false };
        return can_move_dir(map, &next, dir);
    }
    if (c == BOX_LEFT || c == BOX_RIGHT) && (*dir == Direction::Up || *dir == Direction:: Down) {
        let other = if c == BOX_LEFT { map.move_dir(point, &Direction::Right) } else { map.move_dir(point, &Direction::Left) }; 
        let Some(other) = other else { return false };

        let Some(next) = map.move_dir(point, dir) else { return false };
        let Some(next_other) = map.move_dir(&other, dir) else { return false };
        return can_move_dir(map, &next, dir) && can_move_dir(map, &next_other, dir);
    }
    if c == BOX_LEFT || c == BOX_RIGHT {
        let Some(next) = map.move_dir(point, dir) else { return false };
        return can_move_dir(map, &next, dir);
    }
    return false;
}
