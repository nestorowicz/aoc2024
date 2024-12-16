use std::io::stdin;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Point {
    pub y: usize,
    pub x: usize
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        return Point{x, y};
    }
    pub fn move_up(&self) -> Option<Point> {
        return Some(Point{y: self.y.checked_sub(1)?, x: self.x});
    }
    pub fn move_down(&self) -> Option<Point> {
        return Some(Point{y: self.y+1, x: self.x});
    }
    pub fn move_left(&self) -> Option<Point> {
        return Some(Point{y: self.y, x: self.x.checked_sub(1)?});
    }
    pub fn move_right(&self) -> Option<Point> {
        return Some(Point{y: self.y, x: self.x+1});
    }
    pub fn move_in_direction(&self, dir: &Direction) -> Option<Point> {
        match dir {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::UpLeft => self.move_left()?.move_up(),
            Direction::UpRight => self.move_right()?.move_up(),
            Direction::DownLeft => self.move_left()?.move_down(),
            Direction::DownRight => self.move_right()?.move_down(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft
}

impl Direction {
    pub fn rotate_90_clockwise(&self) -> Direction {
        return match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::UpLeft => Direction::UpRight,
            Direction::UpRight => Direction::DownRight,
            Direction::DownRight => Direction::DownLeft,
            Direction::DownLeft => Direction::UpLeft
        }
    }
}

#[derive(Debug)]
pub struct Ray {
    pub point: Point,
    pub direction: Direction
}

impl Ray {
    pub fn new(point: Point, direction: Direction) -> Ray {
        return Ray{point, direction};
    }
    pub fn move_by_one(&self) -> Option<Ray> {
        return Some(Ray{point: self.point.move_in_direction(&self.direction)?, direction: self.direction});
    }
    pub fn rotate_90_clockwise(&self) -> Ray {
        return Ray{point: self.point, direction: self.direction.rotate_90_clockwise()}
    }
    pub fn rotate_90_counter_clockwise(&self) -> Ray {
        return Ray{point: self.point, direction: self.direction.rotate_90_clockwise().rotate_90_clockwise().rotate_90_clockwise()}
    }
}

#[derive(Debug)]
pub struct Map<T> {
    pub state: Vec<Vec<T>>
}

impl<T> Map<T> where
    T:  Debug + PartialEq {
    pub fn peek(&self, point: &Point) -> Option<&T> {
        return Some(self.state.get(point.y)?.get(point.x)?);
    }
    pub fn move_up(&self, point: &Point) -> Option<Point> {
        return Some(Point{y: point.y.checked_sub(1)?, x: point.x});
    }
    pub fn move_down(&self, point: &Point) -> Option<Point> {
        self.state.get(point.y+1)?.get(point.x)?;
        return Some(Point{y: point.y+1, x: point.x});
    }
    pub fn move_left(&self, point: &Point) -> Option<Point> {
        return Some(Point{y: point.y, x: point.x.checked_sub(1)?});
    }
    pub fn move_right(&self, point: &Point) -> Option<Point> {
        self.state.get(point.y)?.get(point.x+1)?;
        return Some(Point{y: point.y, x: point.x+1});
    }
    pub fn move_dir(&self, point: &Point, dir: &Direction) -> Option<Point> {
        match dir {
            Direction::Up => self.move_up(point),
            Direction::Down => self.move_down(point),
            Direction::Left => self.move_left(point),
            Direction::Right => self.move_right(point),
            Direction::UpLeft => self.move_left(&self.move_up(point)?),
            Direction::UpRight => self.move_right(&self.move_up(point)?),
            Direction::DownLeft => self.move_left(&self.move_down(point)?),
            Direction::DownRight => self.move_right(&self.move_down(point)?),
        }
    }
    pub fn height(&self) -> usize {
        return self.state.len();
    }
    pub fn width(&self) -> usize {
        return if let Some(line) = self.state.get(0) { line.len() } else { 0 };
    }
    pub fn iter_points(&self) -> impl Iterator<Item = Point> + '_ {
        return self.state.iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().map(move |(x, _)| Point{x, y})
            });
    }
    pub fn find_position(&self, expected: &T) -> Option<Point> {
        return self.iter_points()
            .filter_map(|point| Some((point, self.peek(&point)?)))
            .find(|(_, element)| expected == *element)
            .map(|(point, _)| point);
    }
}

// This code assumes that the input is correct
pub fn parse_map(lines: &mut std::str::Lines) -> Map<char> {
    let state: Vec<Vec<char>> = lines.take_while(|l| !l.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    return Map{state};
}
pub fn print_char_map(map: &Map<char>) {
    for line in map.state.iter() {
        for c in line.iter() {
            print!("{}", *c);
        }
        println!();
    }
}

pub fn read_line() -> String {
    let mut inp = String::new();
    _ = stdin().read_line(&mut inp).ok().expect("could not read stdin");
    return inp
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_works() {
        let map = parse_map(&mut "12\n34".lines());

        assert_eq!(map.state.len(), 2);
        assert_eq!(map.state[0], vec!['1', '2']);
        assert_eq!(map.state[1], vec!['3', '4']);

        let top_none = map.move_up(&Point{x: 0, y: 0});
        assert!(top_none.is_none());

        let top = map.move_up(&Point{x: 0, y: 1});
        assert!(top.is_some());

        let right = map.move_right(&Point{x: 0, y: 0});
        assert!(right.is_some());

        let right_none = map.move_right(&Point{x: 1, y: 0});
        println!("{:?}", right_none);
        assert!(right_none.is_none());

        let left = map.move_left(&Point{x: 1, y: 0});
        assert!(left.is_some());

        let left_none = map.move_left(&Point{x: 0, y: 0});
        assert!(left_none.is_none());

        let down = map.move_down(&Point{x: 0, y: 0});
        assert!(down.is_some());

        let down_none = map.move_down(&Point{x: 0, y: 1});
        assert!(down_none.is_none());
    }
}
