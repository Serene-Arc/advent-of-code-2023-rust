mod game;

use crate::game::Game;
use std::io::stdin;

fn main() {
    let lines = stdin().lines();
    let games = lines.map(|l| Game::game_from_line(l.unwrap()));

    let mut total = 0;
    for game in games {
        total += game.get_power();
        println!("{}", total);
    }
}
