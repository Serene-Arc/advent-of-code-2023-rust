pub struct Observations {
    observed_values: Vec<isize>,
}

impl Observations {
    pub fn from_line(line: String) -> Self {
        let numbers = line
            .split(' ')
            .map(|n| n.trim().parse::<isize>().unwrap())
            .collect();
        Self {
            observed_values: numbers,
        }
    }

    pub fn extrapolate(&self) -> isize {
        let mut first_digits: Vec<isize> = Vec::new();
        let mut level = self.observed_values.clone();
        loop {
            let recent_level = Self::reduce_series(level);
            if recent_level.iter().all(|v| v == &0) {
                break;
            } else {
                first_digits.push(*recent_level.first().unwrap());
            }
            level = recent_level;
        }
        self.observed_values.first().unwrap()
            - first_digits
                .iter()
                .map(|d| *d)
                .rev()
                .reduce(|a, b| b - a)
                .unwrap()
    }

    fn reduce_series(number_series: Vec<isize>) -> Vec<isize> {
        number_series.windows(2).map(|w| w[1] - w[0]).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::observations::Observations;

    #[test]
    fn test_simple_series_1() {
        let test = Observations::from_line("1 2 3 4".to_string());
        let result = test.extrapolate();
        assert_eq!(result, 0)
    }

    #[test]
    fn test_simple_series_2() {
        let test = Observations::from_line("1 3 5 7".to_string());
        let result = test.extrapolate();
        assert_eq!(result, -1)
    }

    #[test]
    fn test_simple_series_example_1() {
        let test = Observations::from_line("0 3 6 9 12 15".to_string());
        let result = test.extrapolate();
        assert_eq!(result, -3)
    }

    #[test]
    fn test_simple_series_example_2() {
        let test = Observations::from_line("1 3 6 10 15 21".to_string());
        let result = test.extrapolate();
        assert_eq!(result, 0)
    }

    #[test]
    fn test_simple_series_example_3() {
        let test = Observations::from_line("10 13 16 21 30 45".to_string());
        let result = test.extrapolate();
        assert_eq!(result, 5)
    }
}
