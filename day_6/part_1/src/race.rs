pub struct RaceStrategy {
    button_time: usize,
    winning_distance: usize,
    race_time: usize,
}

impl RaceStrategy {
    pub fn new(button_time: usize, winning_distance: usize, race_time: usize) -> Self {
        Self {
            button_time,
            winning_distance,
            race_time,
        }
    }
    pub fn wins_race(&self) -> bool {
        let distance_travelled = self.calculate_distance();
        distance_travelled > self.winning_distance
    }

    fn calculate_distance(&self) -> usize {
        let remaining_time = self.race_time - self.button_time;
        let distance_travelled = remaining_time * self.button_time;
        distance_travelled
    }
}

#[cfg(test)]
mod test {
    use crate::race::RaceStrategy;

    #[test]
    fn test_win_race_1() {
        let test = RaceStrategy::new(1, 7, 7);
        let result = test.wins_race();
        assert_eq!(result, false)
    }

    #[test]
    fn test_win_race_2() {
        let test = RaceStrategy::new(1, 7, 8);
        let result = test.wins_race();
        assert_eq!(result, false)
    }

    #[test]
    fn test_win_race_3() {
        let test = RaceStrategy::new(1, 7, 9);
        let result = test.wins_race();
        assert_eq!(result, true)
    }

    #[test]
    fn test_win_race_4() {
        let test = RaceStrategy::new(3, 9, 7);
        let result = test.wins_race();
        assert_eq!(result, true)
    }

    #[test]
    fn test_win_race_5() {
        let test = RaceStrategy::new(6, 9, 7);
        let result = test.wins_race();
        assert_eq!(result, false)
    }

    #[test]
    fn test_calculate_distance_no_button(){
        let test = RaceStrategy::new(0,10,10);
        let result = test.calculate_distance();
        assert_eq!(result, 0)
    }

    #[test]
    fn test_calculate_distance_all_button(){
        let test = RaceStrategy::new(10,10,10);
        let result = test.calculate_distance();
        assert_eq!(result, 0)
    }

    #[test]
    fn test_calculate_distance_one_second(){
        let test = RaceStrategy::new(1,10,10);
        let result = test.calculate_distance();
        assert_eq!(result, 9)
    }

    #[test]
    fn test_calculate_distance_two_seconds(){
        let test = RaceStrategy::new(2,10,10);
        let result = test.calculate_distance();
        assert_eq!(result, 16)
    }

}
