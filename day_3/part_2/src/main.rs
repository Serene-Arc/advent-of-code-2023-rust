extern crate core;

mod part;

use crate::part::Part;
use ndarray::{s, Array, Ix2};
use std::io;

fn main() {
    let reader = io::stdin();
    let lines: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|l| l.len() > 0)
        .collect();

    let (line_length, number_lines) = (lines[0].len(), lines.len());

    let mut numbers: Array<Option<char>, Ix2> = Array::default((number_lines, line_length));
    let mut valid: Array<bool, Ix2> = Array::default((number_lines, line_length));

    for (i, line) in lines.iter().enumerate() {
        let (temp_numbers, temp_valid) = get_matrices(line);

        numbers
            .slice_mut(s![i, ..])
            .assign(&Array::from(temp_numbers));
        valid.slice_mut(s![i, ..]).assign(&Array::from(temp_valid));
    }

    let valid_locations: Vec<(usize, usize)> = valid
        .indexed_iter()
        .filter_map(|(l, v)| v.then_some(l))
        .collect();

    let mut parts = Vec::new();
    for (i, row) in numbers.rows().into_iter().enumerate() {
        let mut buffer = String::new();
        let mut places = Vec::new();

        for (j, &place) in row.iter().enumerate() {
            match place {
                Some(character) => {
                    buffer.push(character);
                    places.push((i, j));
                }
                None => {
                    if buffer.len() > 0 {
                        parts.push(Part::new(buffer.parse::<usize>().unwrap(), places.clone()));
                    }
                    buffer.clear();
                    places.clear();
                }
            }
        }
        if buffer.len() > 0 {
            parts.push(Part::new(buffer.parse::<usize>().unwrap(), places.clone()));
        }
    }

    let mut total: usize = 0;
    let mut numbers = Vec::new();
    for (x, y) in valid_locations {
        for part in &parts {
            let min = part
                .character_places
                .iter()
                .map(|(a, b)| euclid_distance(*a as i32, *b as i32, x as i32, y as i32))
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            if min <= 1.5f64 {
                numbers.push(part.number);
            }
        }
        if numbers.len() == 2 {
            total += numbers.iter().product::<usize>();
        }
        numbers.clear();
    }

    println!("{}", total);
}

fn euclid_distance(a: i32, b: i32, x: i32, y: i32) -> f64 {
    (((a - x) as f64).powi(2) + ((b - y) as f64).powi(2)).sqrt()
}

fn get_matrices(string: &String) -> (Vec<Option<char>>, Vec<bool>) {
    let mut characters: Vec<Option<char>> = Vec::new();
    let mut symbols = Vec::new();
    for char in string.as_bytes().iter().map(|c| char::from(*c)) {
        if char == '.' {
            characters.push(None);
            symbols.push(false);
        } else if char.is_digit(10) {
            characters.push(Some(char));
            symbols.push(false);
        } else if char == '*' {
            characters.push(None);
            symbols.push(true);
        } else {
            characters.push(None);
            symbols.push(false);
        }
    }
    (characters, symbols)
}
