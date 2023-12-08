use lazy_static::lazy_static;
use std::collections::HashMap;

use advent2023::advent_utils::get_lines_from_filepath;

#[derive(Debug, Clone)]
pub struct CamelCardHand {
    cards: String,
    bid: i64,
}
#[derive(Debug, Clone)]
pub struct CamelCardHandJoker {
    cards: String,
    bid: i64,
}

lazy_static! {
    static ref CARD_ORDER_MAP: HashMap<char, i64> = {
        let mut m = HashMap::new();
        m.insert('A', 1);
        m.insert('K', 2);
        m.insert('Q', 3);
        m.insert('J', 4);
        m.insert('T', 5);
        m.insert('9', 6);
        m.insert('8', 7);
        m.insert('7', 8);
        m.insert('6', 9);
        m.insert('5', 10);
        m.insert('4', 11);
        m.insert('3', 12);
        m.insert('2', 13);
        m
    };
}
lazy_static! {
    static ref CARD_ORDER_MAP_JOKER: HashMap<char, i64> = {
        let mut m = HashMap::new();
        m.insert('A', 1);
        m.insert('K', 2);
        m.insert('Q', 3);
        m.insert('T', 5);
        m.insert('9', 6);
        m.insert('8', 7);
        m.insert('7', 8);
        m.insert('6', 9);
        m.insert('5', 10);
        m.insert('4', 11);
        m.insert('3', 12);
        m.insert('2', 13);
        m.insert('J', 14);
        m
    };
}

impl CamelCardHand {
    fn cards_counter(&self) -> HashMap<char, i64> {
        let mut cards_counter: HashMap<char, i64> = HashMap::new();
        for card in self.cards.chars() {
            cards_counter
                .entry(card)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        cards_counter
    }
}

impl PartialEq for CamelCardHand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for CamelCardHand {}

impl PartialOrd for CamelCardHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelCardHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut self_cards_counter: Vec<(char, i64)> = self
            .cards_counter()
            .iter()
            .map(|(&key, &val)| (key, val))
            .collect::<Vec<(char, i64)>>();
        self_cards_counter.sort_by(|a, b| b.1.cmp(&a.1));
        let mut other_cards_counter: Vec<(char, i64)> = other
            .cards_counter()
            .iter()
            .map(|(&key, &val)| (key, val))
            .collect::<Vec<(char, i64)>>();
        other_cards_counter.sort_by(|a, b| b.1.cmp(&a.1));
        // for i in 0..std::cmp::max(self_cards_counter.len(), other_cards_counter.len()) {
        //     if self_cards_counter[i].1 > other_cards_counter[i].1 {
        //         return std::cmp::Ordering::Greater;
        //     } else if self_cards_counter[i].1 < other_cards_counter[i].1 {
        //         return std::cmp::Ordering::Less;
        //     } else {
        //         continue;
        //     }
        // }
        if self_cards_counter[0].1 > other_cards_counter[0].1 {
            return std::cmp::Ordering::Greater;
        } else if self_cards_counter[0].1 < other_cards_counter[0].1 {
            return std::cmp::Ordering::Less;
        } else if self_cards_counter[1].1 > other_cards_counter[1].1 {
            return std::cmp::Ordering::Greater;
        } else if self_cards_counter[1].1 < other_cards_counter[1].1 {
            return std::cmp::Ordering::Less;
        }
        for i in 0..self.cards.chars().count() {
            let self_card: char = self.cards.chars().nth(i).unwrap();
            let other_card: char = other.cards.chars().nth(i).unwrap();
            if CARD_ORDER_MAP.get(&self_card) < CARD_ORDER_MAP.get(&other_card) {
                return std::cmp::Ordering::Greater;
            } else if CARD_ORDER_MAP.get(&self_card) > CARD_ORDER_MAP.get(&other_card) {
                return std::cmp::Ordering::Less;
            } else {
                continue;
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl CamelCardHandJoker {
    fn cards_counter(&self) -> HashMap<char, i64> {
        let mut cards_counter: HashMap<char, i64> = HashMap::new();
        for card in self.cards.chars() {
            cards_counter
                .entry(card)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        if cards_counter.contains_key(&'J') {
            let mut max_count = cards_counter.values().max().unwrap();
            let jack_count = *cards_counter.get(&'J').unwrap();
            let card_values: Vec<i64> = cards_counter
                .iter()
                .filter(|(&key, _)| key != 'J')
                .map(|(_, &val)| val)
                .collect();
            if max_count == &jack_count && card_values.len() > 0 {
                max_count = card_values.iter().max().unwrap();
            }
            let mut key_to_update: Option<char> = None;
            for (key, val) in cards_counter.iter() {
                if *val == *max_count && *key != 'J' {
                    key_to_update = Some(*key);
                    break;
                }
            }
            if let Some(key) = key_to_update {
                cards_counter
                    .entry(key)
                    .and_modify(|counter| *counter += jack_count);
            }
            //cards_counter.entry('J').and_modify(|counter| *counter = 0);
        }
        cards_counter
    }
}

impl PartialEq for CamelCardHandJoker {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for CamelCardHandJoker {}

impl PartialOrd for CamelCardHandJoker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelCardHandJoker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut self_cards_counter: Vec<(char, i64)> = self
            .cards_counter()
            .iter()
            .filter(|(&key, &_)| key != 'J')
            .map(|(&key, &val)| (key, val))
            .collect::<Vec<(char, i64)>>();
        self_cards_counter.sort_by(|a, b| b.1.cmp(&a.1));
        let mut other_cards_counter: Vec<(char, i64)> = other
            .cards_counter()
            .iter()
            .filter(|(&key, &_)| key != 'J')
            .map(|(&key, &val)| (key, val))
            .collect::<Vec<(char, i64)>>();
        other_cards_counter.sort_by(|a, b| b.1.cmp(&a.1));

        if self_cards_counter.len() == 0 || other_cards_counter.len() == 0 {
            if self_cards_counter.len() == 0 && other_cards_counter.len() == 0 {
                return std::cmp::Ordering::Equal;
            } else if self_cards_counter.len() == 0 {
                if other_cards_counter[0].1 == 5 {
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Greater;
                }
            } else if other_cards_counter.len() == 0 {
                if self_cards_counter[0].1 == 5 {
                    return std::cmp::Ordering::Greater;
                } else {
                    return std::cmp::Ordering::Less;
                }
            }
        } else if self_cards_counter[0].1 > other_cards_counter[0].1 {
            return std::cmp::Ordering::Greater;
        } else if self_cards_counter[0].1 < other_cards_counter[0].1 {
            return std::cmp::Ordering::Less;
        } else if (self_cards_counter.len() == 1 || other_cards_counter.len() == 1)
            || (self_cards_counter[0].1 == 5 && other_cards_counter[0].1 == 5)
        {
        } else if self_cards_counter[1].1 > other_cards_counter[1].1 {
            return std::cmp::Ordering::Greater;
        } else if self_cards_counter[1].1 < other_cards_counter[1].1 {
            return std::cmp::Ordering::Less;
        }
        for i in 0..self.cards.chars().count() {
            let self_card: char = self.cards.chars().nth(i).unwrap();
            let other_card: char = other.cards.chars().nth(i).unwrap();
            if CARD_ORDER_MAP_JOKER.get(&self_card) < CARD_ORDER_MAP_JOKER.get(&other_card) {
                return std::cmp::Ordering::Greater;
            } else if CARD_ORDER_MAP_JOKER.get(&self_card) > CARD_ORDER_MAP_JOKER.get(&other_card) {
                return std::cmp::Ordering::Less;
            } else {
                continue;
            }
        }
        std::cmp::Ordering::Equal
    }
}

fn get_hands(lines: &[String]) -> Vec<CamelCardHand> {
    let mut hands: Vec<CamelCardHand> = Vec::new();
    for line in lines {
        let mut split_line = line.split_whitespace();
        let cards: String = split_line.next().unwrap().to_string();
        let bid: i64 = split_line.next().unwrap().parse::<i64>().unwrap();
        hands.push(CamelCardHand {
            cards: cards,
            bid: bid,
        });
    }
    hands
}

fn get_hands_jokers(lines: &[String]) -> Vec<CamelCardHandJoker> {
    let mut hands: Vec<CamelCardHandJoker> = Vec::new();
    for line in lines {
        let mut split_line = line.split_whitespace();
        let cards: String = split_line.next().unwrap().to_string();
        let bid: i64 = split_line.next().unwrap().parse::<i64>().unwrap();
        hands.push(CamelCardHandJoker {
            cards: cards,
            bid: bid,
        });
    }
    hands
}

fn part_one(lines: &[String]) {
    let mut score: i64 = 0;
    let mut hands: Vec<CamelCardHand> = get_hands(&lines);
    hands.sort_by(|a, b| a.cmp(b));
    for (i, hand) in hands.iter().enumerate() {
        score += hand.bid * (i as i64 + 1);
    }
    println!("{}", score);
}

fn part_two(lines: &[String]) {
    let mut score: i64 = 0;
    let mut hands: Vec<CamelCardHandJoker> = get_hands_jokers(&lines);
    hands.sort_by(|a, b| a.cmp(b));
    for (i, hand) in hands.iter().enumerate() {
        score += hand.bid * (i as i64 + 1);
    }
    println!("{}", score);
}

fn main() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_seven_input.txt");
    part_one(&lines);
    part_two(&lines);
}

#[test]
fn test_camel_card_hand_equality() {
    let hand_one: CamelCardHand = CamelCardHand {
        cards: "AAKKQ".to_string(),
        bid: 123,
    };
    let hand_two: CamelCardHand = CamelCardHand {
        cards: "AAKKQ".to_string(),
        bid: 123,
    };
    assert_eq!(hand_one, hand_two);
}

#[test]
fn test_camel_card_hand_inequality() {
    let hand_one: CamelCardHand = CamelCardHand {
        cards: "AKQJT9".to_string(),
        bid: 123,
    };
    let hand_two: CamelCardHand = CamelCardHand {
        cards: "AKQJT8".to_string(),
        bid: 123,
    };
    let hand_three: CamelCardHand = CamelCardHand {
        cards: "CCAAB".to_string(),
        bid: 456,
    };
    assert_ne!(hand_one, hand_two);
    assert_ne!(hand_one, hand_three);
    assert_ne!(hand_two, hand_three);
}

#[test]
fn test_camel_card_ord() {
    let hand_one: CamelCardHand = CamelCardHand {
        cards: "AKQJT".to_string(),
        bid: 123,
    };
    let hand_two: CamelCardHand = CamelCardHand {
        cards: "AAKQJ".to_string(),
        bid: 123,
    };
    let hand_three: CamelCardHand = CamelCardHand {
        cards: "AAKQJ".to_string(),
        bid: 456,
    };
    let hand_four: CamelCardHand = CamelCardHand {
        cards: "22234".to_string(),
        bid: 123,
    };
    let hand_five: CamelCardHand = CamelCardHand {
        cards: "JQKAA".to_string(),
        bid: 456,
    };
    assert!(hand_one < hand_two);
    assert!(hand_one < hand_three);
    assert!(hand_two == hand_three);
    assert!(hand_four > hand_two);
    assert!(hand_five != hand_three);
}

#[test]
fn test_camel_card_hand_jack_edge_cases() {
    let hand_one: CamelCardHandJoker = CamelCardHandJoker {
        cards: "JJJJJ".to_string(),
        bid: 123,
    };
    let hand_two: CamelCardHandJoker = CamelCardHandJoker {
        cards: "J3J58".to_string(),
        bid: 123,
    };
    assert!(hand_one > hand_two);
}

#[test]
fn test_camel_card_four_of_a_kind() {
    let hand_one: CamelCardHandJoker = CamelCardHandJoker {
        cards: "AAAA2".to_string(),
        bid: 123,
    };
    let hand_two: CamelCardHandJoker = CamelCardHandJoker {
        cards: "JJJ2T".to_string(),
        bid: 123,
    };
    println!("{:?}, {:?}", hand_one, hand_one.cards_counter());
    println!("{:?}, {:?}", hand_two, hand_two.cards_counter());
    assert!(hand_one > hand_two);
}
