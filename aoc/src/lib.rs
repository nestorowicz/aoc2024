#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Point {
    pub y: usize,
    pub x: usize
}

#[derive(Debug)]
pub struct Map<T> {
    pub state: Vec<Vec<T>>
}

impl<T> Map<T> {
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
            });}
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
