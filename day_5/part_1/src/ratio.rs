use std::num::ParseIntError;
pub struct Ratio {
    destination_start: isize,
    source_start: isize,
    range_length: isize,
}

#[derive(Debug)]
pub enum RatioError {
    LessThanThreeParts,
    ParseIntError(ParseIntError),
    CannotMapValue,
}

impl From<ParseIntError> for RatioError {
    fn from(err: ParseIntError) -> RatioError {
        RatioError::ParseIntError(err)
    }
}

impl Ratio {
    fn new(destination_start: isize, source_start: isize, range_length: isize) -> Self {
        Self {
            destination_start,
            source_start,
            range_length,
        }
    }
    pub fn map_value(&self, value: &isize) -> Result<isize, RatioError> {
        if !self.in_range(value) {
            return Err(RatioError::CannotMapValue);
        } else {
            let diff = (self.source_start - value).abs();
            Ok(self.destination_start + diff)
        }
    }
    pub fn from_string(input: String) -> Result<Self, RatioError> {
        let parts: Result<Vec<isize>, RatioError> = input
            .split_whitespace()
            .map(|p| p.parse::<isize>().map_err(RatioError::ParseIntError))
            .collect();

        let parts = parts?;

        if parts.len() < 3 {
            return Err(RatioError::LessThanThreeParts);
        }

        let destination_start = parts[0];
        let source_start = parts[1];
        let range_length = parts[2];

        Ok(Ratio::new(destination_start, source_start, range_length))
    }
    pub fn in_range(&self, value: &isize) -> bool {
        value >= &self.source_start && value < &(self.source_start + self.range_length)
    }
}

#[cfg(test)]
mod test {
    use crate::ratio::Ratio;

    #[test]
    fn test_map_value_simple_1() {
        let test = Ratio::new(50, 98, 2);
        let result = test.map_value(98);
        assert_eq!(result, 50)
    }

    #[test]
    fn test_map_value_simple_2() {
        let test = Ratio::new(50, 98, 2);
        let result = test.map_value(99);
        assert_eq!(result, 51)
    }
    #[test]
    fn test_map_value_simple_3() {
        let test = Ratio::new(52, 50, 48);
        let result = test.map_value(53);
        assert_eq!(result, 55)
    }

    #[test]
    fn test_map_value_simple_outside_range() {
        let test = Ratio::new(50, 52, 48);
        let result = test.map_value(10);
        assert_eq!(result, 10)
    }

    #[test]
    fn test_in_range_lower_boundary() {
        let test = Ratio::new(50, 98, 2);
        let result = test.in_range(&98);
        assert_eq!(result, true)
    }

    #[test]
    fn test_in_range_upper_boundary_in() {
        let test = Ratio::new(50, 98, 2);
        let result = test.in_range(&99);
        assert_eq!(result, true)
    }

    #[test]
    fn test_in_range_upper_boundary_out() {
        let test = Ratio::new(50, 98, 2);
        let result = test.in_range(&100);
        assert_eq!(result, false)
    }

    #[test]
    fn test_in_range_middle() {
        let test = Ratio::new(50, 98, 20);
        let result = test.in_range(&110);
        assert_eq!(result, true)
    }

    #[test]
    fn test_in_range_before() {
        let test = Ratio::new(50, 98, 20);
        let result = test.in_range(&1);
        assert_eq!(result, false)
    }

    #[test]
    fn test_in_range_after() {
        let test = Ratio::new(50, 98, 20);
        let result = test.in_range(&1000);
        assert_eq!(result, false)
    }

    #[test]
    fn test_ratio_from_string_1() {
        let test = Ratio::from_string("50 98 2".to_string());
        assert_eq!(test.unwrap().destination_start, 50)
    }
}
