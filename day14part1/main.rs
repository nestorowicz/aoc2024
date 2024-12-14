use std::io::{read_to_string, stdin};
use scanf::sscanf;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
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

// const WIDTH: i64 = 11;
const WIDTH: i64 = 101;
// const HEIGHT: i64 = 7;
const HEIGHT: i64 = 103;
const SECONDS: i64 = 100;
// const SECONDS: i64 = 5;
const MID_X: i64 = WIDTH/2;
const MID_Y: i64 = HEIGHT/2;

fn main() {
    let mut robots = parse_robots();
    progress_time(&mut robots);
    let quadrants = calculate_quandrants(&robots);

    let answer: i64 = quadrants.iter().product();
    println!("{}", answer);
}

fn parse_robots() -> Vec<Robot> {
    let mut robots: Vec<Robot> = Vec::new();

    let input = read_to_string(stdin()).unwrap();
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

fn progress_time(robots: &mut Vec<Robot>) {
    for robot in robots.iter_mut() {
        robot.position.x += robot.velocity.x * SECONDS;
        robot.position.x = (robot.position.x % WIDTH + WIDTH) % WIDTH;
        robot.position.y += robot.velocity.y * SECONDS;
        robot.position.y = (robot.position.y % HEIGHT + HEIGHT) % HEIGHT;
    }
}

fn calculate_quandrants(robots: &Vec<Robot>) -> Vec<i64> {
    let (l, r): (Vec<Robot>, Vec<Robot>) = robots.iter()
        .filter(|r| r.position.x != MID_X && r.position.y != MID_Y)
        .partition(|r| r.position.x < MID_X);
    let lr = vec![l, r];
    return lr.iter().map(|v| v.iter().partition(|r| r.position.y < MID_Y))
        .flat_map(|(a, b)| vec![a, b])
        .map(|q: Vec<Robot>| q.len() as i64)
        .collect();
}

