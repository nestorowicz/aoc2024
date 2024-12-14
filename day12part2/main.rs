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
    if let Some((area, perimeter)) = visit(garden, mem, &Point{y, x}) {
        return area * perimeter;
    }
    return 0;
}

fn has_symbol(dir: Option<(Point, char)>, symbol: &char) -> bool {
    return dir.is_some() && dir.unwrap().1 == *symbol
}

fn visit(garden: &Vec<Vec<char>>, mem: &mut HashSet<Point>, point: &Point) -> Option<(u64, u64)> {
    let symbol = garden.get(point.y)?.get(point.x)?;
    if mem.contains(&point) {
        return None
    }
    mem.insert(point.clone());

    let up = garden.move_up(&point);
    let right = garden.move_right(&point);
    let down = garden.move_down(&point);
    let left = garden.move_left(&point);
    let up_right = if up.is_some() { garden.move_right(&up.unwrap().0) } else { None };
    let up_left = if up.is_some() { garden.move_left(&up.unwrap().0) } else { None };
    let down_right = if down.is_some() { garden.move_right(&down.unwrap().0) } else { None };
    let down_left = if down.is_some() { garden.move_left(&down.unwrap().0) } else { None };

    let mut perimeter = 0;
    if !has_symbol(up, symbol) && !has_symbol(right, symbol) {
        perimeter += 1;
    }
    if !has_symbol(up, symbol) && !has_symbol(left, symbol) {
        perimeter += 1;
    }
    if !has_symbol(down, symbol) && !has_symbol(right, symbol) {
        perimeter += 1;
    }
    if !has_symbol(down, symbol) && !has_symbol(left, symbol) {
        perimeter += 1;
    }

    if has_symbol(up, symbol) && has_symbol(right, symbol) && !has_symbol(up_right, symbol) {
        perimeter += 1;
    }
    if has_symbol(up, symbol) && has_symbol(left, symbol)  && !has_symbol(up_left, symbol){
        perimeter += 1;
    }
    if has_symbol(down, symbol) && has_symbol(right, symbol)  && !has_symbol(down_right, symbol){
        perimeter += 1;
    }
    if has_symbol(down, symbol) && has_symbol(left, symbol)  && !has_symbol(down_left, symbol){
        perimeter += 1;
    }

    let dirs = vec![up, right, down, left];
    let neighbors: Vec<&(Point, char)> = dirs
        .iter()
        .flatten()
        .filter(|(_, c)| *c == *symbol)
        .collect();

    let (area, perimeter) = neighbors
        .iter()
        .filter_map(|(next, _)| visit(garden, mem, next))
        .fold((1, perimeter), |(area, perimeter), (a, p)| (area+a, perimeter+p));

    return Some((area as u64, perimeter as u64));
}

