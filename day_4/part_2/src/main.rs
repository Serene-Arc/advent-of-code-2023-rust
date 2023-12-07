use std::collections::HashMap;
use crate::ticket::Ticket;
use std::io::stdin;

mod ticket;

fn main() {
    let tickets: HashMap<i32,Ticket> = stdin()
        .lines()
        .filter_map(Result::ok)
        .filter(|l| l.len() > 0)
        .map(|l| Ticket::from_string(l))
        .map(|t| (t.ticket_id, t)).collect();
    let mut queue: Vec<&Ticket> = tickets.values().map(|t| t).collect();
    let mut n = 0;
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        n +=1;
        for i in (current.ticket_id+1)..(current.ticket_id +1 + current.get_winning_numbers().len() as i32){
            queue.push(&tickets[&i]);
        }
    }
    println!("{}", n);
}
