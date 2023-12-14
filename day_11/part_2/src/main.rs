mod coordinate;

use crate::coordinate::Coordinate;
use itertools::Itertools;
use ndarray::{Array2, Axis};
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

    let (empty_rows, empty_columns) = find_empty_columns_and_rows(&initial_matrix);
    let galaxy_locations: Vec<Coordinate> = initial_matrix
        .indexed_iter()
        .filter_map(|((y, x), character)| {
            if character == &'#' {
                Some(Coordinate::new(x as isize, y as isize))
            } else {
                None
            }
        })
        .map(|coordinate| expand_coordinate(coordinate, &empty_rows, &empty_columns, 999_999))
        .collect();
    let galaxy_pairs = galaxy_locations.iter().combinations(2);
    let sum: usize = galaxy_pairs.map(|a| a[0].manhattan_distance(a[1])).sum();
    println!("{}", sum)
}

fn find_empty_indexes(array: &Array2<char>, axis: Axis) -> Vec<usize> {
    array
        .axis_iter(axis)
        .enumerate()
        .filter_map(|(i, line)| {
            if line.iter().all(|&c| c == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

fn find_empty_columns_and_rows(array: &Array2<char>) -> (Vec<usize>, Vec<usize>) {
    let empty_rows = find_empty_indexes(array, Axis(0));
    let empty_columns = find_empty_indexes(array, Axis(1));
    (empty_rows, empty_columns)
}

fn expand_coordinate(
    coordinate: Coordinate,
    expanded_rows: &[usize],
    expanded_columns: &[usize],
    expansion_factor: usize,
) -> Coordinate {
    let empty_rows_before = expanded_rows
        .iter()
        .filter(|&&n| n <= coordinate.y as usize)
        .count()
        * expansion_factor;
    let empty_columns_before = expanded_columns
        .iter()
        .filter(|&&n| n <= coordinate.x as usize)
        .count()
        * expansion_factor;
    Coordinate::new(
        coordinate.x + empty_columns_before as isize,
        coordinate.y + empty_rows_before as isize,
    )
}

#[cfg(test)]
mod test {
    use crate::coordinate::Coordinate;
    use crate::{expand_coordinate, find_empty_columns_and_rows};
    use ndarray::arr2;

    #[test]
    fn test_find_empty_columns_one_row() {
        let test = arr2(&[['.', '#']]);
        let (rows, columns) = find_empty_columns_and_rows(&test);
        assert_eq!(rows, vec![]);
        assert_eq!(columns, vec![0]);
    }

    #[test]
    fn test_find_empty_columns_two_rows() {
        let test = arr2(&[['.', '#'], ['.', '#']]);
        let (rows, columns) = find_empty_columns_and_rows(&test);
        assert_eq!(rows, vec![]);
        assert_eq!(columns, vec![0]);
    }

    #[test]
    fn test_find_empty_columns_two_two_rows() {
        let test = arr2(&[['.', '.', '#'], ['.', '.', '#']]);
        let (rows, columns) = find_empty_columns_and_rows(&test);
        assert_eq!(rows, vec![]);
        assert_eq!(columns, vec![0, 1]);
    }

    #[test]
    fn test_find_empty_rows_one_column() {
        let test = arr2(&[['.'], ['#']]);
        let (rows, columns) = find_empty_columns_and_rows(&test);
        assert_eq!(rows, vec![0]);
        assert_eq!(columns, vec![]);
    }

    #[test]
    fn test_find_empty_rows_two_column() {
        let test = arr2(&[['.', '.'], ['#', '#']]);
        let (rows, columns) = find_empty_columns_and_rows(&test);
        assert_eq!(rows, vec![0]);
        assert_eq!(columns, vec![]);
    }

    #[test]
    fn test_find_empty_rows_and_columns() {
        let test = arr2(&[['.', '.', '.'], ['#', '#', '.']]);
        let (rows, columns) = find_empty_columns_and_rows(&test);
        assert_eq!(rows, vec![0]);
        assert_eq!(columns, vec![2]);
    }

    #[test]
    fn test_expand_coordinate_one_row() {
        let test_coordinate = Coordinate::new(1, 1);
        let result = expand_coordinate(test_coordinate, &*vec![0], &*vec![], 1);
        assert_eq!(result, Coordinate::new(1, 2))
    }

    #[test]
    fn test_expand_coordinate_one_column() {
        let test_coordinate = Coordinate::new(1, 1);
        let result = expand_coordinate(test_coordinate, &*vec![], &*vec![0], 1);
        assert_eq!(result, Coordinate::new(2, 1))
    }
}
