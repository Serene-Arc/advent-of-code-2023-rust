use std::io;
use ndarray::{Array, Array2};

fn main() {
    let lines = io::stdin().lines();
    let line_length = 141;
    let mut number_matrix = Array::new();
    for line in lines{
        match line {
            Ok(line_content) => {
                let characters: Vec<char> = line_content.as_bytes().to_vec();
            }
            Err(_) => {}
        }
    }
}
