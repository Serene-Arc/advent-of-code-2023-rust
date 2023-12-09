use crate::hand::Hand;
use std::io::stdin;

mod hand;

fn main() {
    let lines = stdin()
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| l.len() > 0);
    let mut hands: Vec<(Hand, usize)> = lines
        .map(|s| {
            s.split(' ')
                .map(|ss| ss.trim().to_string())
                .collect::<Vec<String>>()
        })
        .map(|v| {
            (
                Hand::new(v.first().expect("No hand in tuple")),
                v.last()
                    .unwrap()
                    .parse::<usize>()
                    .expect("Can't convert bid number"),
            )
        })
        .collect();
    hands.sort_by(|h1, h2| h1.cmp(h2));
    hands.reverse();
    let mut total: usize = 0;
    for (i, (_, bet)) in hands.iter().enumerate() {
        total += (i + 1) * bet;
    }
    println!("Total winnings are {}", total)
}
