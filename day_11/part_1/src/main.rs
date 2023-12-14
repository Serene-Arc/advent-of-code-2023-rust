mod coordinate;

use crate::coordinate::Coordinate;
use itertools::Itertools;
use ndarray::{Array, Array2, ArrayBase, Dim, Ix, Ix2, OwnedRepr};
use std::io::stdin;

fn main() {
    let lines: Vec<char> = stdin()
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| l.len() > 0)
        .flat_map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let initial_matrix =
        Array2::from_shape_vec((140, 140), lines).expect("Could not make array of this shape");

    let result = expand_matrix(initial_matrix);
    let galaxy_locations: Vec<Coordinate> = result
        .indexed_iter()
        .filter_map(|((x, y), character)| {
            if character == &'#' {
                Some(Coordinate::new(x as isize, y as isize))
            } else {
                None
            }
        })
        .collect();
    let galaxy_pairs = galaxy_locations.iter().combinations(2);
    let sum: usize = galaxy_pairs.map(|a| a[0].manhattan_distance(a[1])).sum();
    println!("{}", sum)
}

fn expand_matrix(
    initial_matrix: ArrayBase<OwnedRepr<char>, Ix2>,
) -> ArrayBase<OwnedRepr<char>, Dim<[Ix; 2]>> {
    let mut new_rows: Vec<Vec<char>> = Vec::new();
    for row in initial_matrix.outer_iter() {
        let row_vec: Vec<_> = row.iter().collect();
        let new_row_vec = row_vec.iter().map(|&&c| c).collect::<Vec<char>>();
        new_rows.push(new_row_vec.clone());
        if row_vec.iter().all(|&x| *x == '.') {
            new_rows.push(new_row_vec);
        }
    }

    let intermediate =
        Array2::from_shape_vec((new_rows.len(), initial_matrix.ncols()), new_rows.concat())
            .unwrap();

    // Find if a column is full of '.'
    let mut new_cols: Vec<Vec<char>> = Vec::new();
    for col in intermediate.t().outer_iter() {
        let col_vec: Vec<char> = col.iter().map(|&c| c).collect();
        new_cols.push(col_vec.clone());
        if col_vec.iter().all(|&x| x == '.') {
            new_cols.push(col_vec.clone());
        }
    }
    let new_col_len = new_cols.len();
    let fixed_new_cols = flatten_columns_correctly(new_cols);

    Array::from_shape_vec((intermediate.nrows(), new_col_len), fixed_new_cols).unwrap()
}

fn flatten_columns_correctly(input: Vec<Vec<char>>) -> Vec<char> {
    let out: Vec<char> = (0..input[0].len())
        .flat_map(|i| input.iter().map(move |v| v[i]))
        .collect::<Vec<char>>();
    out
}

#[cfg(test)]
mod test {
    use crate::expand_matrix;
    use ndarray::arr2;

    #[test]
    fn test_expansion_one_column_one_row() {
        let test_array = arr2(&[['.', '#']]);
        let result = expand_matrix(test_array);
        assert_eq!(result, arr2(&[['.', '.', '#']]))
    }

    #[test]
    fn test_expansion_one_column_two_rows() {
        let test_array = arr2(&[['.', '#'], ['.', '#']]);
        let result = expand_matrix(test_array);
        assert_eq!(result, arr2(&[['.', '.', '#'], ['.', '.', '#'],]))
    }

    #[test]
    fn test_expansion_one_column_three_rows() {
        let test_array = arr2(&[['.', '#'], ['.', '#'], ['.', '#']]);
        let result = expand_matrix(test_array);
        assert_eq!(
            result,
            arr2(&[['.', '.', '#'], ['.', '.', '#'], ['.', '.', '#'],])
        )
    }
}
