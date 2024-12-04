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
            if game[i][j] != 'A' {
                continue
            }
            result += find_xmas(&game, i, j)
        }
    }
    println!("{}", result);
}

fn find_xmas(game: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mas_up_left = find(&game, "MAS", decr(i+1), decr(j+1));
    let sam_up_left = find(&game, "SAM", decr(i+1), decr(j+1));
    let mas_up_right = find(&game, "MAS", decr(i+1), incr(j.wrapping_sub(1)));
    let sam_up_right = find(&game, "SAM", decr(i+1), incr(j.wrapping_sub(1)));
    
    if (mas_up_left || sam_up_left) && (mas_up_right || sam_up_right) {
        return 1
    }
    return 0
}

fn incr(i: usize) -> impl Iterator<Item = usize> {
    return (0..).map(move |n| i+n);
}
fn decr(i: usize) -> impl Iterator<Item = usize> {
    return (0..).map(move |n| i.wrapping_sub(n));
}

fn find(game: &Vec<Vec<char>>, word: &str, mut is: impl Iterator<Item = usize>, mut js: impl Iterator<Item = usize>) -> bool {
    let mut chars = word.chars();
    loop {
        let Some(character) = chars.next() else { return true};
        let Some(i) = is.next() else { return false };
        let Some(j) = js.next() else { return false };
        let Some(line) = game.get(i) else { return false };
        let Some(game_char) = line.get(j) else { return false };
        if *game_char != character {
            return false
        }
    }
}

