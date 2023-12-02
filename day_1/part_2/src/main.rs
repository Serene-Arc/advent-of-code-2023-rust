use regex::{Match, Regex};
use std::collections::HashMap;
use std::io;
use std::io::Error;

fn main() {
    let input = io::stdin().lines();
    let mut total: usize = 0;
    for line in input {
        let i = extract_numbers_from_line(line);
        println!("{}", i);
        total += i;
    }
    println!("{}", total);
}

fn extract_numbers_from_line(line: Result<String, Error>) -> usize {
    let number_regexes = [
        Regex::new(r"[0-9]").unwrap(),
        Regex::new(r"one").unwrap(),
        Regex::new(r"two").unwrap(),
        Regex::new(r"three").unwrap(),
        Regex::new(r"four").unwrap(),
        Regex::new(r"five").unwrap(),
        Regex::new(r"six").unwrap(),
        Regex::new(r"seven").unwrap(),
        Regex::new(r"eight").unwrap(),
        Regex::new(r"nine").unwrap(),
    ];
    let number_map: HashMap<&str, String> = HashMap::from([
        ("one", "1".to_string()),
        ("two", "2".to_string()),
        ("three", "3".to_string()),
        ("four", "4".to_string()),
        ("five", "5".to_string()),
        ("six", "6".to_string()),
        ("seven", "7".to_string()),
        ("eight", "8".to_string()),
        ("nine", "9".to_string()),
    ]);
    let interim_line = line.unwrap().trim().to_string().to_lowercase();
    let mut found_digits: Vec<Match> = number_regexes
        .iter()
        .flat_map(|r| r.find_iter(interim_line.as_str()))
        .collect();
    found_digits.sort_by(|m1, m2| m1.start().partial_cmp(&m2.start()).unwrap());

    if found_digits.len() > 0 {
        let mut digits = [
            found_digits.first().unwrap().as_str(),
            found_digits.last().unwrap().as_str(),
        ];
        for index in 0..2 {
            if number_map.contains_key(digits[index]) {
                digits[index] = number_map[digits[index]].as_str();
            }
        }
        return format!("{}{}", digits[0], digits[1],)
            .parse::<usize>()
            .unwrap();
    }
    0
}

#[cfg(test)]
mod test {

    #[test]
    fn test_two_numbers() {
        let result = super::extract_numbers_from_line(Ok("f96xhv".to_string()));
        assert_eq!(result, 96)
    }

    #[test]
    fn test_one_number_end_string() {
        let result = super::extract_numbers_from_line(Ok("5seven7slxxbsjqktseven".to_string()));
        assert_eq!(result, 57)
    }

    #[test]
    fn test_one_number_start_string() {
        let result = super::extract_numbers_from_line(Ok("six97".to_string()));
        assert_eq!(result, 67)
    }

    #[test]
    fn test_one_number() {
        let result = super::extract_numbers_from_line(Ok("d6".to_string()));
        assert_eq!(result, 66)
    }

    #[test]
    fn test_overlapping_strings_first() {
        let result =
            super::extract_numbers_from_line(Ok("jtwonetwothree5znqsvfour5czgsqvvtgg".to_string()));
        assert_eq!(result, 25)
    }

    #[test]
    fn test_overlapping_strings_last() {
        let result = super::extract_numbers_from_line(Ok("1twone".to_string()));
        assert_eq!(result, 11)
    }
}
