use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::prelude::*;
use std::time::Instant;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    JOKER
}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type < other.hand_type {
            return Ordering::Less;
        } else if self.hand_type > other.hand_type {
            return Ordering::Greater;
        } else {
            for (index, card) in self.cards.iter().enumerate() {
                if card < &other.cards[index] {
                    return Ordering::Less;
                } else if card > &other.cards[index] {
                    return Ordering::Greater;
                }
            }
            return Ordering::Greater;
        }
    }
}

fn parse_hand_part_one (cards_vec: &Vec<Card>) -> HandType {
    let mut cards_counts: HashMap<Card, u32> = HashMap::new();

    for card in cards_vec {
        if cards_counts.contains_key(card) {
            continue;
        } else {
            cards_counts.insert(
                *card,
                cards_vec
                    .iter()
                    .filter(|card_to_check| *card_to_check == card)
                    .count() as u32,
            );
        }
    }

    if cards_counts.values().count() == 1 {
        return HandType::FiveOfAKind
    }

    if cards_counts.values().count() == 2 {
        for card_count in cards_counts.values() {
            if card_count == &4 {
                return HandType::FourOfAKind;
            }
            if card_count == &3 {
                return HandType::FullHouse;
            }
        }
    }

    if cards_counts.values().count() == 3 {
        let mut not_two_pair: bool = false;
        for card_count in cards_counts.values() {
            if card_count > &2 {
                not_two_pair = true;
            }
        }
        if not_two_pair {
            return HandType::ThreeOfAKind;
        } else {
            return HandType::TwoPair;
        }
    }

    if cards_counts.values().count() == 4 {
        return HandType::OnePair;
    }

    if cards_counts.values().count() == 5 {
        return HandType::HighCard;
    }

    return HandType::FiveOfAKind; // unreachable code
}

fn parse_hand_part_two (cards_vec: &Vec<Card>) -> HandType {
    let mut cards_counts: HashMap<Card, u32> = HashMap::new();
    let jokers_amount: u32 = cards_vec.iter().filter(|card| { *card == &Card::JOKER }).count() as u32;

    for card in cards_vec {
        if cards_counts.contains_key(card) {
            continue;
        } else {
            cards_counts.insert(
                *card,
                cards_vec
                    .iter()
                    .filter(|card_to_check| *card_to_check == card)
                    .count() as u32,
            );
        }
    }

    if cards_counts.values().count() == 1 {
        return HandType::FiveOfAKind
    }

    if cards_counts.values().count() == 2 { // QQQAA
        if jokers_amount > 0 { // JJJAA => AAAAA || AAAJJ => AAAAA || JAAAA => AAAAA || JJJJA => AAAAA
            return HandType::FiveOfAKind;
        }
        for card_count in cards_counts.values() {
            if card_count == &4 { // AAAAQ
                return HandType::FourOfAKind;
            }
            if card_count == &3 {
                return HandType::FullHouse;
            }
        }
    }

    if cards_counts.values().count() == 3 {
        let mut not_two_pair: bool = false;
        for card_count in cards_counts.values() {
            if card_count > &2 {
                not_two_pair = true;
            }
        }
        if not_two_pair { // QQQA2
            if jokers_amount > 0 {
                return HandType::FourOfAKind;
            }
            return HandType::ThreeOfAKind;
        } else {
            if jokers_amount > 0 {
                if jokers_amount == 1 { // AAJQQ => AAAQQ
                    return HandType::FullHouse;
                }
                if jokers_amount == 2 { // JJAQQ => QQAQQ
                    return HandType::FourOfAKind;
                }
            }
            return HandType::TwoPair;
        }
    }

    if cards_counts.values().count() == 4 { // QA27Q
        if jokers_amount == 1 { // QAJ7Q => QAQ7Q
            return HandType::ThreeOfAKind;
        }
        if jokers_amount == 2 { // JJQA2 => QQQA2
            return HandType::ThreeOfAKind;
        }
        return HandType::OnePair;
    }

    if cards_counts.values().count() == 5 {
        if jokers_amount > 0 {
            return HandType::OnePair;
        }
        return HandType::HighCard;
    }

    panic!("Something bad happened") // unreachable code
}

impl Hand {
    fn new(cards: String, bid: u32, part: u32) -> Self {
        let cards_vec: Vec<Card> = cards
            .chars()
            .map(|card| {
                match card {
                    'A' => Card::A,
                    'K' => Card::K,
                    'Q' => Card::Q,
                    'J' => if part == 1 { Card::J} else { Card::JOKER } ,
                    'T' => Card::T,
                    '2' => Card::Two,
                    '3' => Card::Three,
                    '4' => Card::Four,
                    '5' => Card::Five,
                    '6' => Card::Six,
                    '7' => Card::Seven,
                    '8' => Card::Eight,
                    '9' => Card::Nine,
                    _ => Card::A, // handle impossible behaviour
                }
            })
            .collect();

        let hand_type = if part == 1 { parse_hand_part_one(&cards_vec) } else { parse_hand_part_two(&cards_vec) };

        return Self {
            cards: cards_vec,
            hand_type,
            bid
        }
    }
}

fn part_one() {
    let mut input = aoclib::file_reader::get_input("day7".to_string());

    let mut sum: u64 = 0;

    let start_time = Instant::now();

    let mut hands: Vec<Hand> = Vec::new();

    for line in input.by_ref().lines() {
        let line = line.unwrap();

        let (hand, bid) = line.split_once(" ").unwrap();

        hands.push(Hand::new(hand.to_string(), bid.parse::<u32>().unwrap(), 1),)
    }

    hands.sort_by(|hand, other| hand.cmp(other));
    hands.reverse();

    for (index, hand) in hands.iter().enumerate() {
        sum += (index + 1) as u64 * hand.bid as u64;
    }

    println!(
        "Finished part 1 in {:#?} with answer {:#?}",
        start_time.elapsed(),
        sum
    )
}

fn part_two() {
    let mut input = aoclib::file_reader::get_input("day7".to_string());

    let mut sum: u64 = 0;

    let start_time = Instant::now();

    let mut hands: Vec<Hand> = Vec::new();

    for line in input.by_ref().lines() {
        let line = line.unwrap();

        let (hand, bid) = line.split_once(" ").unwrap();

        hands.push(Hand::new(hand.to_string(), bid.parse::<u32>().unwrap(), 2),)
    }

    hands.sort_by(|hand, other| hand.cmp(other));
    hands.reverse();

    for (index, hand) in hands.iter().enumerate() {
        sum += (index + 1) as u64 * hand.bid as u64;
    }

    println!(
        "Finished part 2 in {:#?} with answer {:#?}",
        start_time.elapsed(),
        sum
    )
}

fn main() {
    println!("Solving day 7...");
    part_one();
    part_two()
}