use std::cmp::{min, max};
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
const PRESSES_LIMIT: u64 = 100;

fn main() {
    let games = parse_games();
    println!("Games: {}", games.len());
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

        let game = Game{button_a: Coord{x: button_a_x, y: button_a_y}, button_b: Coord{x: button_b_x, y: button_b_y}, prize: Coord{x: prize_x, y: prize_y}};
        games.push(game);
    }

    return games;
}

fn find_fewest_tokens(game: &Game) -> Option<u64> {
    let btn_a = &game.button_a;
    let btn_b = &game.button_b;
    let prize = &game.prize;
    let mut cheapest: Option<u64> = None;
    for a_presses in 0..min(max(prize.x / btn_a.x, prize.y / btn_a.y), PRESSES_LIMIT) {
        let a_x = a_presses * btn_a.x;
        let a_y = a_presses * btn_a.y;
        if a_x > prize.x || a_y > prize.y {
            continue
        }

        let b_presses = min(max((prize.x - a_x)/btn_b.x, (prize.y-a_y)/btn_b.y), PRESSES_LIMIT);

        let b_x = b_presses * btn_b.x;
        let b_y = b_presses * btn_b.y;

        if (a_x + b_x) == prize.x && (a_y + b_y) == prize.y {
            let price = a_presses * BUTTON_A_PRICE + b_presses * BUTTON_B_PRICE;
            if cheapest.is_none() || cheapest.unwrap() > price {
                cheapest = Some(price);
            }
        }
    }
    return cheapest
}
