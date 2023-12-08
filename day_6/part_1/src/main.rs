use crate::race::RaceStrategy;
use std::io::stdin;

mod race;

fn main() {
    let lines: Vec<String> = stdin().lines().filter_map(|l| l.ok()).filter(|s| s.len() > 0).collect();
    let times = lines
        .first()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter_map(|t| t.parse::<usize>().ok());
    let record_scores = lines
        .last()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split(' ')
        .filter_map(|t| t.parse::<usize>().ok());
    let pairs = times.zip(record_scores);
    let strategies: usize = pairs
        .map(|(time, record)| {
            (1..time)
                .map(move |b| RaceStrategy::new(b, record, time))
                .filter(|r| r.wins_race())
                .collect()
        }).map(|v: Vec<RaceStrategy>| v.len()).product();

    println!("{}", strategies)
}
