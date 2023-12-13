mod coordinate;

use crate::coordinate::Coordinate;
use crate::CardinalDirection::{EAST, NORTH, SOUTH, WEST};
use ndarray::Array2;
use phf::phf_map;
use std::io::stdin;

static PIPES: phf::Map<char, (CardinalDirection, CardinalDirection)> = phf_map! {
    '|' => (NORTH, SOUTH),
    '-' => (EAST, WEST),
    'L' => (NORTH, EAST),
    'J' => (NORTH, WEST),
    '7' => (SOUTH, WEST),
    'F' => (SOUTH, EAST),
};

fn main() {
    let characters: Vec<char> = stdin()
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| l.len() > 0)
        .collect::<Vec<String>>()
        .iter()
        .flat_map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let map: Array2<char> =
        Array2::from_shape_vec((140, 140), characters).expect("Could not form array");
    let mut history: Vec<Coordinate> = Vec::new();
    let start = map
        .indexed_iter()
        .find_map(
            |((x, y), &character)| {
                if character == 'S' {
                    Some((x, y))
                } else {
                    None
                }
            },
        )
        .unwrap();
    history.push(Coordinate::new(start.1 as isize, start.0 as isize));
    let mut current_coord = Coordinate::new((start.1 + 1) as isize, start.0 as isize);
    loop {
        let next = get_next_coordinate_change_from_char(
            *map.get((current_coord.y as usize, current_coord.x as usize))
                .expect("Cannot get character"),
            &current_coord,
            &history,
        );
        history.push(current_coord.clone());
        if &next == history.first().unwrap() {
            break;
        } else {
            current_coord = next
        }
    }
    println!("{}", history.len())
}

#[derive(Eq, PartialEq, Debug)]
enum CardinalDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl CardinalDirection {
    pub fn value(&self) -> (isize, isize) {
        match *self {
            CardinalDirection::NORTH => (0, -1),
            CardinalDirection::EAST => (1, 0),
            CardinalDirection::SOUTH => (0, 1),
            CardinalDirection::WEST => (-1, 0),
        }
    }
    pub fn from_value(value: (isize, isize)) -> CardinalDirection {
        match value {
            (0, -1) => CardinalDirection::NORTH,
            (1, 0) => CardinalDirection::EAST,
            (0, 1) => CardinalDirection::SOUTH,
            (-1, 0) => CardinalDirection::WEST,
            _ => panic!("No direction for this value"),
        }
    }
    pub fn invert(&self) -> CardinalDirection {
        match self {
            NORTH => SOUTH,
            SOUTH => NORTH,
            EAST => WEST,
            WEST => EAST,
        }
    }
}

fn get_next_coordinate_change_from_char(
    character: char,
    current_place: &Coordinate,
    history: &Vec<Coordinate>,
) -> Coordinate {
    let entering_direction = get_entering_direction(current_place, history);
    let direction_tuple = PIPES
        .get(&character)
        .expect("Character does not exist in map");
    let new_direction = if entering_direction == direction_tuple.0 {
        &direction_tuple.1
    } else if entering_direction == direction_tuple.1 {
        &direction_tuple.0
    } else {
        panic!("Destination not in tuple")
    };
    current_place.move_to(new_direction.value())
}

fn get_entering_direction(
    current_place: &Coordinate,
    history: &Vec<Coordinate>,
) -> CardinalDirection {
    let entering_direction = CardinalDirection::from_value(
        current_place.relative_to(
            history
                .last()
                .expect("Can't get last historical coordinate"),
        ),
    )
    .invert();
    entering_direction
}

#[cfg(test)]
mod test {
    use crate::coordinate::Coordinate;
    use crate::CardinalDirection::{EAST, NORTH, SOUTH, WEST};
    use crate::{get_entering_direction, get_next_coordinate_change_from_char};

    #[test]
    fn test_get_entering_direction_south() {
        let test_current_place = Coordinate::new(0, 0);
        let history = vec![Coordinate::new(0, 1)];
        let result = get_entering_direction(&test_current_place, &history);
        assert_eq!(result, SOUTH)
    }

    #[test]
    fn test_get_entering_direction_north() {
        let test_current_place = Coordinate::new(0, 1);
        let history = vec![Coordinate::new(0, 0)];
        let result = get_entering_direction(&test_current_place, &history);
        assert_eq!(result, NORTH)
    }

    #[test]
    fn test_get_entering_direction_east() {
        let test_current_place = Coordinate::new(0, 0);
        let history = vec![Coordinate::new(1, 0)];
        let result = get_entering_direction(&test_current_place, &history);
        assert_eq!(result, EAST)
    }

    #[test]
    fn test_get_entering_direction_west() {
        let test_current_place = Coordinate::new(1, 0);
        let history = vec![Coordinate::new(0, 0)];
        let result = get_entering_direction(&test_current_place, &history);
        assert_eq!(result, WEST)
    }

    #[test]
    fn test_next_coordinate_vertical_from_south() {
        let test_char = '|';
        let test_current_place = Coordinate::new(0, 1);
        let history = vec![Coordinate::new(0, 0)];
        let result = get_next_coordinate_change_from_char(test_char, &test_current_place, &history);
        assert_eq!(result, Coordinate::new(0, 2))
    }

    #[test]
    fn test_next_coordinate_horizontal_from_east() {
        let test_char = '|';
        let test_current_place = Coordinate::new(21, 27);
        let history = vec![Coordinate::new(21, 28)];
        let result = get_next_coordinate_change_from_char(test_char, &test_current_place, &history);
        assert_eq!(result, Coordinate::new(21, 26))
    }

    #[test]
    fn test_next_coordinate_j_bend_from_north() {
        let test_char = 'J';
        let test_current_place = Coordinate::new(0, 1);
        let history = vec![Coordinate::new(0, 0)];
        let result = get_next_coordinate_change_from_char(test_char, &test_current_place, &history);
        assert_eq!(result, Coordinate::new(-1, 1))
    }

    #[test]
    fn test_next_coordinate_j_bend_from_west() {
        let test_char = 'J';
        let test_current_place = Coordinate::new(1, 0);
        let history = vec![Coordinate::new(0, 0)];
        let result = get_next_coordinate_change_from_char(test_char, &test_current_place, &history);
        assert_eq!(result, Coordinate::new(1, -1))
    }
}
