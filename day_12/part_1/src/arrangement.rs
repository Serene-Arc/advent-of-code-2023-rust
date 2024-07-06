#[derive(Eq, PartialEq, Debug)]
enum STATE {
    OPERATIONAL,
    DAMAGED,
    UNKNOWN,
}
pub struct Arrangement {
    state_history: Vec<STATE>,
    size_history: Vec<usize>,
}

impl Arrangement {
    fn new(state_history: Vec<STATE>, size_history: Vec<usize>) -> Self {
        Self {
            state_history,
            size_history,
        }
    }
    pub fn from_line(line: String) -> Self {
        let first_parts: Vec<&str> = line.split(' ').collect();
        let state_history = first_parts
            .first()
            .unwrap()
            .chars()
            .map(|c| match c {
                '#' => STATE::DAMAGED,
                '.' => STATE::OPERATIONAL,
                '?' => STATE::UNKNOWN,
                _ => {
                    panic!("Got an unknown character")
                }
            })
            .collect();
        let size_history = first_parts
            .last()
            .unwrap()
            .split(',')
            .map(|c| {
                c.parse::<usize>()
                    .expect("Could not convert character to usize")
            })
            .collect();
        Self::new(state_history, size_history)
    }
    pub fn resolve_missing_records(&self) {
        //
    }
}

#[cfg(test)]
mod test {
    use crate::arrangement::Arrangement;
    use crate::arrangement::STATE::*;

    #[test]
    fn test_from_string_one_record() {
        let result = Arrangement::from_line("# 1".to_string());
        assert_eq!(result.state_history.len(), 1);
        assert_eq!(result.size_history.len(), 1);
    }

    #[test]
    fn test_from_string_two_records() {
        let result = Arrangement::from_line("## 2".to_string());
        assert_eq!(result.state_history.len(), 2);
        assert_eq!(result.size_history.len(), 1);
    }

    #[test]
    fn test_from_string_three_records_two_groups() {
        let result = Arrangement::from_line("#.# 1,1".to_string());
        assert_eq!(result.state_history.len(), 3);
        assert_eq!(result.size_history.len(), 2);
    }

    #[test]
    fn test_from_string_three_records_two_groups_one_unknown() {
        let result = Arrangement::from_line("#?# 1,1".to_string());
        assert_eq!(result.state_history.len(), 3);
        assert_eq!(result.size_history.len(), 2);
        assert_eq!(result.state_history[1], UNKNOWN);
    }

    #[test]
    fn test_resolve_missing_one_arrangement_one_end_group() {
        let test_arrangement = Arrangement::from_line("?# 2".to_string());
        test_arrangement.resolve_missing_records();
        assert_eq!(test_arrangement.state_history, vec![DAMAGED, DAMAGED])
    }
}
