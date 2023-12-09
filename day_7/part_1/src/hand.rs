use phf::phf_map;
use std::cmp::Ordering;
use std::collections::HashMap;

static CARDS: phf::Map<char, usize> = phf_map! {
    '2' => 1,
    '3' => 2,
    '4' => 3,
    '5' => 4,
    '6' => 5,
    '7' => 6,
    '8' => 7,
    '9' => 8,
    'J' => 10,
    'T' => 9,
    'Q' => 11,
    'K' => 12,
    'A' => 13,
};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Eq, PartialEq)]
pub struct Hand {
    cards: Vec<char>,
    card_bins: HashMap<char, usize>,
    hand_type: HandType,
}

impl Hand {
    pub fn new(characters: &str) -> Self {
        let mut card_bins: HashMap<char, usize> = HashMap::new();
        for character in characters.chars() {
            *card_bins.entry(character).or_insert(0) += 1;
        }
        let cards = characters.chars().collect();
        let hand_type = Self::get_hand_type(&card_bins);
        Self {
            cards,
            card_bins,
            hand_type,
        }
    }

    fn get_hand_type(card_bins: &HashMap<char, usize>) -> HandType {
        if card_bins.len() == 1 {
            HandType::FiveKind
        } else if card_bins.len() == 2 {
            // Could be a full-house or four-of-a-kind
            if *card_bins.values().max().expect("Could not find maximum") == 4 {
                HandType::FourKind
            } else {
                HandType::FullHouse
            }
        } else if card_bins.len() == 3 {
            if *card_bins.values().max().expect("Could not find maximum") == 3 {
                HandType::ThreeKind
            } else {
                HandType::TwoPair
            }
        } else if card_bins.len() == 4 {
            // Here there are four cards
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_compare = self.hand_type.cmp(&other.hand_type);
        if hand_compare == Ordering::Equal {
            for (i, character_value) in self
                .cards
                .iter()
                .map(|c| CARDS.get(c).expect("Could not find card in map"))
                .enumerate()
            {
                let character_order = character_value.cmp(
                    CARDS
                        .get(&other.cards[i])
                        .expect("Could not find card in map"),
                );
                if !(character_order == Ordering::Equal) {
                    return character_order.reverse();
                }
            }
            Ordering::Equal
        } else {
            hand_compare
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use crate::hand::{Hand, HandType};
    use std::cmp::Ordering;
    use std::collections::HashMap;

    #[test]
    fn test_cmp_hand_five_kind_vs_high_card() {
        let first = Hand::new("AAAAA");
        let second = Hand::new("23456");
        let result = second.cmp(&first);
        assert_eq!(result, Ordering::Greater)
    }

    #[test]
    fn test_cmp_hand_five_kind_vs_four_kind() {
        let first = Hand::new("AAAAA");
        let second = Hand::new("AAAA2");
        let result = second.cmp(&first);
        assert_eq!(result, Ordering::Greater)
    }

    #[test]
    fn test_cmp_hand_high_card_first() {
        let first = Hand::new("34567");
        let second = Hand::new("64567");
        let result = second.cmp(&first);
        assert_eq!(result, Ordering::Less)
    }

    #[test]
    fn test_cmp_hand_high_card_last() {
        let first = Hand::new("34569");
        let second = Hand::new("34567");
        let result = second.cmp(&first);
        assert_eq!(result, Ordering::Greater)
    }

    #[test]
    fn test_cmp_hand_three_kind_high_card_middle() {
        let first = Hand::new("44467");
        let second = Hand::new("44457");
        let result = second.cmp(&first);
        assert_eq!(result, Ordering::Greater)
    }

    #[test]
    fn test_get_hand_type_five_kind() {
        let test = HashMap::from([('A', 5)]);
        let result = Hand::get_hand_type(&test);
        assert_eq!(result, HandType::FiveKind)
    }

    #[test]
    fn test_get_hand_type_four_kind() {
        let test = HashMap::from([('A', 4), ('2', 1)]);
        let result = Hand::get_hand_type(&test);
        assert_eq!(result, HandType::FourKind)
    }

    #[test]
    fn test_get_hand_type_full_house() {
        let test = HashMap::from([('A', 3), ('2', 2)]);
        let result = Hand::get_hand_type(&test);
        assert_eq!(result, HandType::FullHouse)
    }

    #[test]
    fn test_get_hand_type_three_kind() {
        let test = HashMap::from([('A', 3), ('2', 1), ('3', 1)]);
        let result = Hand::get_hand_type(&test);
        assert_eq!(result, HandType::ThreeKind)
    }

    #[test]
    fn test_get_hand_type_two_pair() {
        let test = HashMap::from([('A', 2), ('2', 2), ('3', 1)]);
        let result = Hand::get_hand_type(&test);
        assert_eq!(result, HandType::TwoPair)
    }

    #[test]
    fn test_get_hand_type_one_pair() {
        let test = HashMap::from([('A', 2), ('2', 1), ('3', 1), ('4', 1)]);
        let result = Hand::get_hand_type(&test);
        assert_eq!(result, HandType::OnePair)
    }

    #[test]
    fn test_get_hand_type_high_card() {
        let test = HashMap::from([('A', 1), ('2', 1), ('3', 1), ('4', 1), ('5', 1)]);
        let result = Hand::get_hand_type(&test);
        assert_eq!(result, HandType::HighCard)
    }
}
