mod game;

use crate::game::Game;
use std::io::stdin;

fn main() {
    let lines = stdin().lines();
    let games = lines.map(|l| Game::game_from_line(l.unwrap()));
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut total = 0;
    for game in games.filter(|g| {
        g.known_red <= red_limit && g.known_green <= green_limit && g.known_blue <= blue_limit
    }) {
        total += game.id;
        println!("{}", total);
    }
}
