use crate::observations::Observations;
use std::io::stdin;

mod observations;

fn main() {
    let lines = stdin()
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| l.len() > 0);
    let observations = lines.map(|l| Observations::from_line(l));
    let result: isize = observations.map(|o| o.extrapolate()).sum();
    println!("{}", result)
}
