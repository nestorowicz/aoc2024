use std::io::{read_to_string, stdin};

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

const MAX_ALTITUDE: u32= 9;

fn main() {
    let map: Vec<Vec<u32>> = read_to_string(stdin()).unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(1)).collect())
        .collect();

    let height = map.len();
    let width = map[0].len();

    let checksum: usize =
        (0..height).flat_map(|y| (0..width).map(move |x| (x,y)))
        .map(|(x, y)| find_trails(&map, Point{x,y}, 0))
        .map(|v| v.len())
        .sum();

    println!("{:?}", checksum);
}

fn find_trails(map: &Vec<Vec<u32>>, point: Point, altitude: u32) -> Vec<Point> {
    if map[point.y][point.x] != altitude {
        return vec![];
    }
    if map[point.y][point.x] == MAX_ALTITUDE {
       return vec![point];
    }

    return vec![
        move_north(map, point),
        move_east(map, point),
        move_south(map, point),
        move_west(map, point)
    ].into_iter()
        .filter_map(|next| next)
        .map(|next| find_trails(map, next, altitude+1))
        .flatten()
        .collect();
}

fn move_north(_: &Vec<Vec<u32>>, point: Point) -> Option<Point> {
    return Some(Point{x: point.x, y: point.y.checked_sub(1)? })
}
fn move_south(map: &Vec<Vec<u32>>, point: Point) -> Option<Point> {
    map.get(point.y+1)?;
    return Some(Point{x: point.x, y: point.y+1 })
}
fn move_west(_: &Vec<Vec<u32>>, point: Point) -> Option<Point> {
    return Some(Point{x: point.x.checked_sub(1)?, y: point.y })
}
fn move_east(map: &Vec<Vec<u32>>, point: Point) -> Option<Point> {
    map.get(point.y)?.get(point.x+1)?;
    return Some(Point{x: point.x+1, y: point.y })
}
