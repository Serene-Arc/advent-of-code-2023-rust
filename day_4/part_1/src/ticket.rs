pub struct Ticket {
    winning_numbers: Vec<i32>,
    all_numbers: Vec<i32>,
}

impl Ticket {
    pub fn new(winning_numbers: Vec<i32>, all_numbers: Vec<i32>) -> Self {
        Self {
            winning_numbers,
            all_numbers,
        }
    }
    pub fn calculate_worth(&self) -> i32 {
        let mut n: i32 = -1;
        for number in &self.all_numbers {
            if self.winning_numbers.contains(&number) {
                n += 1;
            }
        }
        if n >= 0 {
            return 2_i32.pow(n as u32) as i32;
        }
        0
    }
    pub fn from_string(input: String) -> Self {
        let parts: Vec<String> = input.split([':', '|']).map(|s| s.to_string()).collect();
        Self {
            winning_numbers: get_numbers(&parts[1]),
            all_numbers: get_numbers(&parts[2]),
        }
    }
}

fn get_numbers(input: &String) -> Vec<i32> {
    input
        .split(' ')
        .filter_map(|p| p.parse::<i32>().ok())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::ticket::Ticket;

    #[test]
    fn test_worth_none() {
        let test = Ticket::new(Vec::new(), Vec::new());
        let result = test.calculate_worth();
        assert_eq!(result, 0)
    }

    #[test]
    fn test_worth_no_match() {
        let test = Ticket::new(vec![1], vec![0]);
        let result = test.calculate_worth();
        assert_eq!(result, 0)
    }

    #[test]
    fn test_worth_one_match() {
        let test = Ticket::new(vec![1], vec![0, 1]);
        let result = test.calculate_worth();
        assert_eq!(result, 1)
    }

    #[test]
    fn test_worth_two_matches_same() {
        let test = Ticket::new(vec![1], vec![0, 1, 1]);
        let result = test.calculate_worth();
        assert_eq!(result, 2)
    }

    #[test]
    fn test_worth_two_matches_different() {
        let test = Ticket::new(vec![1, 2], vec![0, 1, 2]);
        let result = test.calculate_worth();
        assert_eq!(result, 2)
    }
}
