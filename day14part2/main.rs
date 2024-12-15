use std::{collections::HashMap, io::stdin};
use scanf::sscanf;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
struct Coord {
    x: i64,
    y: i64
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Robot{
    position: Coord,
    velocity: Coord
}

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;
const SECONDS: i64 = 1;
const MID_X: i64 = WIDTH/2;

fn main() {
    let mut robots = parse_robots();

    let mut seconds = 0;
    loop {
        let symmetry = calculate_symmetry(&robots);
        if symmetry > 0.2 {
            print(&robots);
            println!("{}", seconds);
            println!();
            let mut inp = String::new();
            _ = stdin().read_line(&mut inp).ok().expect("whoops");
            if inp == "q" {
                break;
            }
        }
        progress_time(&mut robots);
        seconds += 1;
    }
}

fn parse_robots() -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();

    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let mut lines = input.lines();
    let mut line = lines.next();
    while line.is_some() {
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let mut v_x: i64 = 0;
        let mut v_y: i64 = 0;
        sscanf!(line.unwrap(), "p={},{} v={},{}", x, y, v_x, v_y).expect("failed to parse robot");
        line = lines.next();

        let robot = Robot{position: Coord{x, y}, velocity: Coord{x: v_x, y: v_y}};
        robots.push(robot);
    }

    return robots;
}

fn print(robots: &Vec<Robot>) {
    let mut map: HashMap<Coord, usize> = HashMap::new();
    for robot in robots {
        if let Some(p) = map.get_mut(&robot.position) {
            *p += 1;
        } else {
            map.insert(robot.position, 1);
        }
    }
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if let Some(robots) = map.get(&Coord{x, y}) {
                print!("{}", robots);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn progress_time(robots: &mut Vec<Robot>) {
    for robot in robots.iter_mut() {
        robot.position.x += robot.velocity.x * SECONDS;
        robot.position.x = (robot.position.x % WIDTH + WIDTH) % WIDTH;
        robot.position.y += robot.velocity.y * SECONDS;
        robot.position.y = (robot.position.y % HEIGHT + HEIGHT) % HEIGHT;
    }
}

fn calculate_symmetry(robots: &Vec<Robot>) -> f64 {
    let mut map: HashMap<Coord, usize> = HashMap::new();
    for robot in robots {
        if let Some(p) = map.get_mut(&robot.position) {
            *p += 1;
        } else {
            map.insert(robot.position, 1);
        }
    }
    let l: Vec<&Coord> = map.keys()
        .filter(|p| p.x < MID_X).collect();
    return l.iter()
        .filter(|k| map.contains_key(&Coord{x: WIDTH - k.x, y: k.y}))
        .count() as f64 / l.len() as f64;
}

