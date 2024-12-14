use std::{collections::HashSet, io::{read_to_string, stdin}};

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Copy)]
struct Point {
    y: usize,
    x: usize
}

trait Garden {
    fn peek(&self, point: Point) -> Option<(Point, char)>;
    fn move_up(&self, point: &Point) -> Option<(Point, char)>;
    fn move_down(&self, point: &Point) -> Option<(Point, char)>;
    fn move_left(&self, point: &Point) -> Option<(Point, char)>;
    fn move_right(&self, point: &Point) -> Option<(Point, char)>;
}

impl Garden for Vec<Vec<char>> {
    fn peek(&self, point: Point) -> Option<(Point, char)> {
        return Some((point, *self.get(point.y)?.get(point.x)?));
    }
    fn move_up(&self, point: &Point) -> Option<(Point, char)> {
        return self.peek(Point{y: point.y.checked_sub(1)?, x: point.x});
    }
    fn move_down(&self, point: &Point) -> Option<(Point, char)> {
        return self.peek(Point{y: point.y+1, x: point.x});
    }
    fn move_left(&self, point: &Point) -> Option<(Point, char)> {
        return self.peek(Point{y: point.y, x: point.x.checked_sub(1)?});
    }
    fn move_right(&self, point: &Point) -> Option<(Point, char)> {
        return self.peek(Point{y: point.y, x: point.x+1});
    }
}

fn main() {
    let garden: Vec<Vec<char>> = read_to_string(stdin())
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let mut sum: u64 = 0;
    let mut mem: HashSet<Point> = HashSet::new();
    for (y, line) in garden.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            sum += calculate_region_price(&garden, &mut mem, y, x)
        }
    }

    println!("{}", sum);
}

fn calculate_region_price(garden: &Vec<Vec<char>>, mem: &mut HashSet<Point>, y: usize, x: usize) -> u64 {
    if let Some((area, perimeter)) = visit(garden, mem, &Point{y, x}, None) {
        println!("Visited region {}: {} {}", garden[y][x], area, perimeter);
        println!();
        return area * perimeter;
    }
    return 0;
}

fn visit(garden: &Vec<Vec<char>>, mem: &mut HashSet<Point>, point: &Point, prev: Option<&Point>) -> Option<(u64, u64)> {
    let symbol = garden.get(point.y)?.get(point.x)?;
    if mem.contains(&point) {
        return None
    }
    mem.insert(point.clone());
    println!("Visiting {:?} prev {:?}", point, prev);

    let up = garden.move_up(&point);
    let right = garden.move_right(&point);
    let down = garden.move_down(&point);
    let left = garden.move_left(&point);

    let prev_up = if prev.is_some() { garden.move_up(&prev.unwrap()) } else { None };
    let prev_right = if prev.is_some() { garden.move_right(&prev.unwrap()) } else { None };
    let prev_down = if prev.is_some() { garden.move_down(&prev.unwrap()) } else { None };
    let prev_left = if prev.is_some() { garden.move_left(&prev.unwrap()) } else { None };

    let mut perimeter = 0;
    if (up.is_none() || up.unwrap().1 != *symbol) && (prev.is_none() || (prev_up.is_some() && prev_up.unwrap().1 == *symbol)) {
        println!("perimeter up");
        perimeter += 1;
    }
    if (right.is_none() || right.unwrap().1 != *symbol) && (prev.is_none() || (prev_right.is_some() && prev_right.unwrap().1 == *symbol)) {
        println!("perimeter right");
        perimeter += 1;
    }
    if (down.is_none() || down.unwrap().1 != *symbol) && (prev.is_none() || (prev_down.is_some() && prev_down.unwrap().1 == *symbol)) {
        println!("perimeter down");
        perimeter += 1;
    }
    if (left.is_none() || left.unwrap().1 != *symbol) && (prev.is_none() || (prev_left.is_some() && prev_left.unwrap().1 == *symbol)) {
        println!("perimeter left");
        perimeter += 1;
    }
    println!("sides {}", perimeter);

    let dirs = vec![up, right, down, left];
    let neighbors: Vec<&(Point, char)> = dirs
        .iter()
        .flatten()
        .filter(|(_, c)| *c == *symbol)
        .collect();

    let (area, perimeter) = neighbors
        .iter()
        .filter_map(|(next, _)| visit(garden, mem, next, Some(point)))
        .map(|(a, p)| { println!("{}, {}", a, p); (a, p) })
        .fold((1, perimeter), |(area, perimeter), (a, p)| (area+a, perimeter+p));

    return Some((area as u64, perimeter as u64));
}

