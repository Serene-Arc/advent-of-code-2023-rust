use num_enum::{IntoPrimitive, TryFromPrimitive};
use phf::phf_map;
use std::cmp::Ordering;
use std::collections::HashMap;

static CARDS: phf::Map<char, usize> = phf_map! {
    'J' => 0,
    '2' => 1,
    '3' => 2,
    '4' => 3,
    '5' => 4,
    '6' => 5,
    '7' => 6,
    '8' => 7,
    '9' => 8,
    'T' => 9,
    'Q' => 10,
    'K' => 11,
    'A' => 12,
};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, TryFromPrimitive, IntoPrimitive, Copy, Clone)]
#[repr(u8)]
enum HandType {
    FiveKind = 0,
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
        let hand_type = Self::get_hand_type_with_jokers(&card_bins);
        Self {
            cards,
            card_bins,
            hand_type,
        }
    }

    fn get_hand_type_with_jokers(card_bins: &HashMap<char, usize>) -> HandType {
        let number_of_jokers = *card_bins.get(&'J').unwrap_or(&0_usize);
        let mut new_bins = card_bins.clone();
        new_bins.remove_entry(&'J');
        let new_bins = new_bins;

        if number_of_jokers == 1 || number_of_jokers == 4 {
            let hands = (0..=number_of_jokers)
                .map(|i| {
                    let mut updated_bins_list = Vec::new();
                    for k in new_bins.keys() {
                        let mut n = new_bins.clone();
                        *n.get_mut(k).unwrap() += i;
                        if (number_of_jokers - i) > 0 {
                            n.insert('J', number_of_jokers - i);
                        }
                        updated_bins_list.push(n);
                    }
                    updated_bins_list
                })
                .flat_map(|bins_list| bins_list.into_iter())
                .map(|h: HashMap<char, usize>| Self::get_hand_type(&h))
                .min()
                .unwrap();
            hands
        } else if number_of_jokers == 5 {
            HandType::FiveKind
        } else if number_of_jokers == 0 {
            Self::get_hand_type(&new_bins)
        } else if number_of_jokers == 2 {
            let mut hands = Vec::new();
            let keys: Vec<&char> = new_bins.keys().collect();
            for (n1, n2) in (0..=number_of_jokers).map(|i| (i, number_of_jokers - i)) {
                let mut cloned_bin = new_bins.clone();
                *cloned_bin.get_mut(keys[0]).unwrap() += n1;
                if cloned_bin.len() >= 2 {
                    *cloned_bin.get_mut(keys[1]).unwrap() += n2;
                }
                hands.push(cloned_bin);
            }
            hands.iter().map(|h| Self::get_hand_type(&h)).min().unwrap()
        } else {
            // There are three jokers then
            if new_bins.len() >= 2 {
                HandType::FourKind
            } else {
                HandType::FiveKind
            }
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
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::FiveKind)
    }

    #[test]
    fn test_get_hand_type_four_kind() {
        let test = HashMap::from([('A', 4), ('2', 1)]);
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::FourKind)
    }

    #[test]
    fn test_get_hand_type_five_kind_one_jack() {
        let test = HashMap::from([('A', 4), ('J', 1)]);
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::FiveKind)
    }

    #[test]
    fn test_get_hand_type_five_kind_two_jacks() {
        let test = HashMap::from([('A', 3), ('J', 2)]);
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::FiveKind)
    }

    #[test]
    fn test_get_hand_type_five_kind_five_jacks() {
        let test = HashMap::from([('J', 5)]);
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::FiveKind)
    }

    #[test]
    fn test_get_hand_type_full_house() {
        let test = HashMap::from([('A', 3), ('2', 2)]);
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::FullHouse)
    }

    #[test]
    fn test_get_hand_type_full_house_jack() {
        let test = HashMap::from([('A', 2), ('2', 2), ('J', 1)]);
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::FullHouse)
    }

    #[test]
    fn test_get_hand_type_three_kind() {
        let test = HashMap::from([('A', 3), ('2', 1), ('3', 1)]);
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::ThreeKind)
    }

    #[test]
    fn test_get_hand_type_three_kind_one_jack() {
        let test = HashMap::from([('A', 2), ('2', 1), ('3', 1), ('J', 1)]);
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::ThreeKind)
    }

    #[test]
    fn test_get_hand_type_three_kind_three_jacks() {
        let test = HashMap::from([('A', 1), ('2', 1), ('J', 3)]);
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::FourKind)
    }

    #[test]
    fn test_get_hand_type_two_pair() {
        let test = HashMap::from([('A', 2), ('2', 2), ('3', 1)]);
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::TwoPair)
    }

    #[test]
    fn test_get_hand_type_one_pair() {
        let test = HashMap::from([('A', 2), ('2', 1), ('3', 1), ('4', 1)]);
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::OnePair)
    }

    #[test]
    fn test_get_hand_type_high_card() {
        let test = HashMap::from([('A', 1), ('2', 1), ('3', 1), ('4', 1), ('5', 1)]);
        let result = Hand::get_hand_type_with_jokers(&test);
        assert_eq!(result, HandType::HighCard)
    }
}
