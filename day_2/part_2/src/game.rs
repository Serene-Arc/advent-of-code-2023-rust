use std::str::FromStr;
use strum::EnumString;

pub struct Game {
    pub id: usize,
    pub known_red: usize,
    pub known_green: usize,
    pub known_blue: usize,
}

#[derive(EnumString, Debug, Eq, PartialEq)]
enum Colour {
    RED,
    GREEN,
    BLUE,
}

impl Game {
    pub fn new(id: usize, red: usize, green: usize, blue: usize) -> Self {
        Game {
            id,
            known_red: red,
            known_green: green,
            known_blue: blue,
        }
    }

    pub fn game_from_line(line: String) -> Self {
        let first_split: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        let id = first_split
            .first()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let colours = Self::get_numbers_and_colours(first_split.last().unwrap());
        let mut largest_red = 0;
        let mut largest_green = 0;
        let mut largest_blue = 0;

        for (size, colour) in colours {
            match colour {
                Colour::RED => largest_red = largest_red.max(size),
                Colour::GREEN => largest_green = largest_green.max(size),
                Colour::BLUE => largest_blue = largest_blue.max(size),
            }
        }
        Game::new(id, largest_red, largest_green, largest_blue)
    }

    fn get_numbers_and_colours(match_string: &str) -> Vec<(usize, Colour)> {
        let pattern = regex::Regex::new(r"\d+ (blue|green|red)").unwrap();
        let mut out: Vec<(usize, Colour)> = vec![];
        for m in pattern.find_iter(match_string.trim()) {
            let parts: Vec<&str> = m.as_str().split(' ').collect();
            let number_part = parts[0].parse::<usize>().unwrap();
            let colour = Colour::from_str(parts[1].to_uppercase().as_str()).unwrap();
            out.push((number_part, colour));
        }
        out
    }
    pub fn get_power(&self) -> usize {
        self.known_blue * self.known_green * self.known_red
    }
}

#[cfg(test)]
mod test {
    use crate::game::Colour;

    #[test]
    fn test_number_colour_parse_1() {
        let result = super::Game::get_numbers_and_colours("5 blue");
        assert_eq!(result, vec![(5, Colour::BLUE)])
    }

    #[test]
    fn test_number_colour_parse_2() {
        let result = super::Game::get_numbers_and_colours("5 blue, 3 green");
        assert_eq!(result, vec![(5, Colour::BLUE), (3, Colour::GREEN)])
    }

    #[test]
    fn test_simple_1() {
        let result = super::Game::game_from_line(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
        );
        assert_eq!(result.id, 1);
        assert_eq!(result.known_red, 4);
        assert_eq!(result.known_green, 2);
        assert_eq!(result.known_blue, 6);
    }
}
