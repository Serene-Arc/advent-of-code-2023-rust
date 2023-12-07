pub struct Part {
    pub number: usize,
    pub character_places: Vec<(usize, usize)>,
}

impl Part {
    pub fn new(number: usize, character_places: Vec<(usize, usize)>) -> Self {
        Self {
            number,
            character_places,
        }
    }
}
