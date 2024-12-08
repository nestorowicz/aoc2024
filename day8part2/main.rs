use std::{collections::{HashMap, HashSet}, io};

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Eq)]
struct Point {
    x: i64,
    y: i64
}

fn main() {
    let input: String = io::read_to_string(io::stdin()).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let columns = lines.get(0).unwrap().len();
    let antennas = parse_map(&lines);

    // How many unique locations within the bounds of the map contain an antinode?
    let result: HashSet<Point> = antennas.iter()
        .flat_map(|(_, points)| find_antinodes(points, rows as i64, columns as i64))
        .collect::<HashSet<Point>>();

    println!("{:?}", result.len());
}

fn parse_map(lines: &Vec<&str>) -> HashMap<char, Vec<Point>> {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            if symbol == '.' {
                continue
            }
            antennas.entry(symbol).or_insert(vec![]).push(Point{x: x as i64, y: y as i64});
        }
    }
    return antennas;
}

fn find_antinodes(points: &Vec<Point>, rows: i64, columns: i64) -> Vec<Point> {
    let mut antinodes: Vec<Point> = vec![];
    for i in 0..points.len()-1 {
        for j in i+1..points.len() {
            antinodes.append(&mut find_harmonic_frequencies(&points[i], &points[j], rows, columns));
        }
    }
    return antinodes;
}

fn find_harmonic_frequencies(p1: &Point, p2: &Point, rows: i64, columns: i64) -> Vec<Point> {
    let mut antinodes: Vec<Point> = vec![*p1, *p2];
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;

    let mut p = p1.clone();
    loop {
        p.x -= dx;
        p.y -= dy;
        if p.x < 0 || p.y < 0 || p.x >= columns || p.y >= rows {
            break;
        }
        antinodes.push(p.clone());
    }

    let mut p = p2.clone();
    loop {
        p.x += dx;
        p.y += dy;
        if p.x < 0 || p.y < 0 || p.x >= columns || p.y >= rows {
            break;
        }
        antinodes.push(p.clone());
    }
    return antinodes;
}
