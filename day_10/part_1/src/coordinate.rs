#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn relative_to(&self, other: &Self) -> (isize, isize) {
        (self.x - other.x, self.y - other.y)
    }
    pub fn move_to(&self, distance: (isize, isize)) -> Self {
        Self {
            x: self.x + distance.0,
            y: self.y + distance.1,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::coordinate::Coordinate;

    #[test]
    fn test_relative_to_same() {
        let test = Coordinate::new(0, 0);
        let result = test.relative_to(&Coordinate::new(0, 0));
        assert_eq!(result, (0, 0))
    }

    #[test]
    fn test_relative_to_right() {
        let test = Coordinate::new(0, 0);
        let result = test.relative_to(&Coordinate::new(1, 0));
        assert_eq!(result, (-1, 0))
    }

    #[test]
    fn test_relative_to_left() {
        let test = Coordinate::new(0, 0);
        let result = test.relative_to(&Coordinate::new(-1, 0));
        assert_eq!(result, (1, 0))
    }

    #[test]
    fn test_relative_to_up() {
        let test = Coordinate::new(0, 0);
        let result = test.relative_to(&Coordinate::new(0, 1));
        assert_eq!(result, (0, -1))
    }

    #[test]
    fn test_relative_to_down() {
        let test = Coordinate::new(0, 0);
        let result = test.relative_to(&Coordinate::new(0, -1));
        assert_eq!(result, (0, 1))
    }
}
