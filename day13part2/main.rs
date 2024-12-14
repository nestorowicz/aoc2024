use std::io::{read_to_string, stdin};
use scanf::sscanf;

#[derive(Debug)]
struct Coord {
    x: u64,
    y: u64
}

#[derive(Debug)]
struct Game{
    button_a: Coord,
    button_b: Coord,
    prize: Coord
}

const BUTTON_A_PRICE: u64 = 3;
const BUTTON_B_PRICE: u64 = 1;

fn main() {
    let games = parse_games();
    let answer: u64 = games.iter().filter_map(|game| find_fewest_tokens(game)).sum();
    println!("{}", answer);
}

fn parse_games() -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    let input = read_to_string(stdin()).unwrap();
    let mut lines = input.lines();
    let mut line = lines.next();
    while line.is_some() {
        let mut button_a_x: u64 = 0;
        let mut button_a_y: u64 = 0;
        sscanf!(line.unwrap(), "Button A: X+{}, Y+{}", button_a_x, button_a_y).expect("failed to parse button a");
        line = lines.next();

        let mut button_b_x: u64 = 0;
        let mut button_b_y: u64 = 0;
        sscanf!(line.unwrap(), "Button B: X+{}, Y+{}", button_b_x, button_b_y).expect("failed to parse button b");
        line = lines.next();

        let mut prize_x: u64 = 0;
        let mut prize_y: u64 = 0;
        sscanf!(line.unwrap(), "Prize: X={}, Y={}", prize_x, prize_y).expect("failed to parse price");
        _ = lines.next();
        line = lines.next();

        prize_x += 10000000000000;
        prize_y += 10000000000000;

        let game = Game{button_a: Coord{x: button_a_x, y: button_a_y}, button_b: Coord{x: button_b_x, y: button_b_y}, prize: Coord{x: prize_x, y: prize_y}};
        games.push(game);
    }

    return games;
}

fn find_fewest_tokens(game: &Game) -> Option<u64> {
    let xp = game.prize.x;
    let yp = game.prize.y;

    let xa = game.button_a.x;
    let ya = game.button_a.y;
    let aa = ya as f64 / xa as f64;

    let xb = game.button_b.x;
    let yb = game.button_b.y;
    let ab = yb as f64 / xb as f64;
    let bb = yp as f64 - ab*xp as f64;

    let x = bb/(aa-ab);
    let y = (aa*bb)/(aa-ab);

    let x = x.round() as u64;
    let y = y.round() as u64;

    if x > xp || y > yp {
        return None
    }

    if  x % xa != 0 || y % ya != 0 {
        return None
    }
    if (xp-x) % xb != 0 || (yp-y)%yb != 0 {
        return None;
    }

    let result: u64 = (y/ya) * BUTTON_A_PRICE + (yp-y)/yb*BUTTON_B_PRICE;
    return Some(result)
}
