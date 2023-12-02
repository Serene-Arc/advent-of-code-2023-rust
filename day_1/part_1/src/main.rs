use regex::{Match, Regex};
use std::io;

fn main() {
    let input = io::stdin().lines();
    let mut total: usize = 0;
    let number_regex = Regex::new(r"[0-9]").unwrap();
    for line in input {
        let interim_line = line.unwrap().trim().to_string();
        let digits: Vec<Match> = number_regex.find_iter(interim_line.as_str()).collect();
        if digits.len() > 0 {
            let interim = format!(
                "{}{}",
                digits.first().unwrap().as_str(),
                digits.last().unwrap().as_str(),
            );
            total += interim.parse::<usize>().unwrap();
        }
    }
    println!("{}", total);
}
