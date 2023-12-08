use crate::race::RaceStrategy;
use std::io::stdin;

mod race;

fn main() {
    let lines: Vec<String> = stdin().lines().filter_map(|l| l.ok()).filter(|s| s.len() > 0).collect();
    let time = lines
        .first()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split(' ').map(|s| s.trim()).collect::<String>().parse::<usize>().unwrap();
    let record_score = lines
        .last()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split(' ').map(|s| s.trim()).collect::<String>().parse::<usize>().unwrap();
    let strategies:usize =
            (1..time)
                .map(move |b| RaceStrategy::new(b, record_score, time))
                .filter(|r| r.wins_race())
                .count();

    println!("{}", strategies)
}
