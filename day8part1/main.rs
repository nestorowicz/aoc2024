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
        .flat_map(|(_, points)| find_antinodes(points))
        .filter(|point| point_in_bound(&point, rows, columns))
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

fn find_antinodes(points: &Vec<Point>) -> Vec<Point> {
    let mut antinodes: Vec<Point> = vec![];
    for i in 0..points.len()-1 {
        for j in i+1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            antinodes.push(Point{
                x: p1.x - dx,
                y: p1.y - dy
            }); 
            antinodes.push(Point{
                x: p2.x + dx,
                y: p2.y + dy
            }); 
        }
    }
    return antinodes;
}

fn point_in_bound(point: &Point, rows: usize, columns: usize) -> bool {
    return point.x >= 0 && (point.x as usize) < columns && point.y >= 0 && (point.y as usize) < rows
}
