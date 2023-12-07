use crate::ticket::Ticket;
use std::io::stdin;

mod ticket;

fn main() {
    let tickets: Vec<Ticket> = stdin()
        .lines()
        .filter_map(Result::ok)
        .filter(|l| l.len() > 0)
        .map(|l| Ticket::from_string(l))
        .collect();
    let total: i32 = tickets.iter().map(|t| t.calculate_worth()).sum();
    println!("{}", total);
}
