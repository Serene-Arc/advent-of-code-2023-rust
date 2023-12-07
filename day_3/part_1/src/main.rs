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
    valid = make_left_right_valid(make_above_below_valid(valid));

    let mut total: usize = 0;
    for (i, row) in numbers.rows().into_iter().enumerate() {
        let mut buffer = String::new();
        let mut valid_flag = false;

        for (j, &place) in row.iter().enumerate() {
            match place {
                Some(character) => {
                    buffer.push(character);
                    if *valid.get((i, j)).unwrap() {
                        valid_flag = true;
                    }
                }
                None if valid_flag => {
                    total += buffer.parse::<usize>().unwrap_or(0);
                    buffer.clear();
                    valid_flag = false;
                }
                None => buffer.clear(),
            }
        }

        if valid_flag {
            total += buffer.parse::<usize>().unwrap_or(0);
        }
    }

    println!("{}", total);
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
        } else {
            characters.push(None);
            symbols.push(true);
        }
    }
    (characters, symbols)
}

fn make_above_below_valid(valid_array: Array<bool, Ix2>) -> Array<bool, Ix2> {
    let mut out = valid_array.clone();
    for ((i, j), v) in out.indexed_iter_mut() {
        *v |= *valid_array.get((i.saturating_sub(1), j)).unwrap_or(&false)
            || *valid_array
                .get((i.checked_add(1).unwrap_or(i), j))
                .unwrap_or(&false);
    }
    out
}

fn make_left_right_valid(valid_array: Array<bool, Ix2>) -> Array<bool, Ix2> {
    let mut out = valid_array.clone();
    for ((i, j), v) in out.indexed_iter_mut() {
        *v |= *valid_array.get((i, j.saturating_sub(1))).unwrap_or(&false)
            || *valid_array
                .get((i, j.checked_add(1).unwrap_or(j)))
                .unwrap_or(&false);
    }
    out
}

#[cfg(test)]
mod test {
    use crate::{get_matrices, make_above_below_valid, make_left_right_valid};
    use ndarray::{array, Array, Ix2};

    #[test]
    fn test_get_matrices_both_empty() {
        let result = get_matrices(&".".to_string());
        assert_eq!(result.0, vec![None]);
        assert_eq!(result.1, vec![false]);
    }

    #[test]
    fn test_get_matrices_one_number() {
        let result = get_matrices(&"1".to_string());
        assert_eq!(result.0, vec![Some('1')]);
        assert_eq!(result.1, vec![false]);
    }

    #[test]
    fn test_get_matrices_one_symbol_hash() {
        let result = get_matrices(&"#".to_string());
        assert_eq!(result.0, vec![None]);
        assert_eq!(result.1, vec![true]);
    }

    #[test]
    fn test_get_matrices_one_symbol_dollar() {
        let result = get_matrices(&"$".to_string());
        assert_eq!(result.0, vec![None]);
        assert_eq!(result.1, vec![true]);
    }

    #[test]
    fn test_get_matrices_2_mix() {
        let result = get_matrices(&"1$".to_string());
        assert_eq!(result.0, vec![Some('1'), None]);
        assert_eq!(result.1, vec![false, true]);
    }

    #[test]
    fn test_get_matrices_3_mix() {
        let result = get_matrices(&"1$2".to_string());
        assert_eq!(result.0, vec![Some('1'), None, Some('2')]);
        assert_eq!(result.1, vec![false, true, false]);
    }

    #[test]
    fn test_check_vertical_valid_above() {
        let mut test: Array<bool, Ix2> = Array::default((3, 1));
        *test.get_mut((0, 0)).unwrap() = true;
        let result = make_above_below_valid(test);
        assert_eq!(result, array![[true], [true], [false]])
    }

    #[test]
    fn test_check_vertical_valid_middle() {
        let mut test: Array<bool, Ix2> = Array::default((3, 1));
        *test.get_mut((1, 0)).unwrap() = true;
        let result = make_above_below_valid(test);
        assert_eq!(result, array![[true], [true], [true]])
    }

    #[test]
    fn test_check_vertical_valid_bottom() {
        let mut test: Array<bool, Ix2> = Array::default((3, 1));
        *test.get_mut((2, 0)).unwrap() = true;
        let result = make_above_below_valid(test);
        assert_eq!(result, array![[false], [true], [true]])
    }

    #[test]
    fn test_check_horizontal_valid_left() {
        let mut test: Array<bool, Ix2> = Array::default((1, 3));
        *test.get_mut((0, 0)).unwrap() = true;
        let result = make_left_right_valid(test);
        assert_eq!(result, array![[true, true, false]])
    }

    #[test]
    fn test_check_horizontal_valid_middle() {
        let mut test: Array<bool, Ix2> = Array::default((1, 3));
        *test.get_mut((0, 1)).unwrap() = true;
        let result = make_left_right_valid(test);
        assert_eq!(result, array![[true, true, true]])
    }

    #[test]
    fn test_check_horizontal_valid_right() {
        let mut test: Array<bool, Ix2> = Array::default((1, 3));
        *test.get_mut((0, 2)).unwrap() = true;
        let result = make_left_right_valid(test);
        assert_eq!(result, array![[false, true, true]])
    }
}
