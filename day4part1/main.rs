use std::fs::read_to_string;

fn main() {
    let game: Vec<Vec<char>> = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut result = 0;
    for (i, line) in game.iter().enumerate() {
        for (j, _) in line.iter().enumerate() {
            if game[i][j] != 'X' {
                continue
            }
            result += find_word(&game, "XMAS", i, j)
        }
    }
    println!("{}", result);
}

fn find_word(game: &Vec<Vec<char>>, word: &str, i: usize, j: usize) -> i32 {
    let mut result: i32 = 0;
    result += find(&game, word, repeat(i), incr(j));
    result += find(&game, word, repeat(i), decr(j));
    result += find(&game, word, incr(i), repeat(j));
    result += find(&game, word, decr(i), repeat(j));
    result += find(&game, word, decr(i), decr(j));
    result += find(&game, word, decr(i), incr(j));
    result += find(&game, word, incr(i), incr(j));
    result += find(&game, word, incr(i), decr(j));
    return result
}

fn incr(i: usize) -> impl Iterator<Item = usize> {
    return (0..).map(move |n| i+n);
}
fn decr(i: usize) -> impl Iterator<Item = usize> {
    return (0..).map(move |n| i.wrapping_sub(n));
}
fn repeat(i: usize) -> impl Iterator<Item = usize> {
    return std::iter::repeat(i);
}

fn find(game: &Vec<Vec<char>>, word: &str, mut is: impl Iterator<Item = usize>, mut js: impl Iterator<Item = usize>) -> i32 {
    let mut chars = word.chars();
    loop {
        let Some(character) = chars.next() else { return 1};
        let Some(i) = is.next() else { return 0 };
        let Some(j) = js.next() else { return 0 };
        let Some(line) = game.get(i) else { return 0 };
        let Some(game_char) = line.get(j) else { return 0 };
        if *game_char != character {
            return 0
        }
    }
}

