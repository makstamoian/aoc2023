use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
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

impl Hand {
    fn new(cards: String, bid: u32) -> Self {
        let mut hand_type: Option<HandType> = None;

        let cards_vec: Vec<Card> = cards
            .chars()
            .map(|card| {
                match card {
                    'A' => Card::A,
                    'K' => Card::K,
                    'Q' => Card::Q,
                    'J' => Card::J,
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

        let mut cards_counts: HashMap<Card, u32> = HashMap::new();

        for card in &cards_vec {
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
            hand_type = Some(HandType::FiveOfAKind);
        }

        if cards_counts.values().count() == 2 {
            for card_count in cards_counts.values() {
                if card_count == &4 {
                    hand_type = Some(HandType::FourOfAKind);
                    break;
                }
                if card_count == &3 {
                    hand_type = Some(HandType::FullHouse);
                    break;
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
                hand_type = Some(HandType::ThreeOfAKind);
            } else {
                hand_type = Some(HandType::TwoPair);
            }
        }

        if cards_counts.values().count() == 4 {
            hand_type = Some(HandType::OnePair);
        }

        if cards_counts.values().count() == 5 {
            hand_type = Some(HandType::HighCard);
        }

        return Self {
            cards: cards_vec,
            bid,
            hand_type: hand_type.unwrap(),
        };
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Card2 {
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
}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
struct Hand2 {
    cards: Vec<Card2>,
    bid: u32,
    hand_type: HandType,
}

impl Ord for Hand2 {
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

impl Hand2 {
    fn new(cards: String, bid: u32) -> Self {
        let mut hand_type: Option<HandType> = None;

        let cards_vec: Vec<Card2> = cards
            .chars()
            .map(|card| {
                match card {
                    'A' => Card2::A,
                    'K' => Card2::K,
                    'Q' => Card2::Q,
                    'J' => Card2::J,
                    'T' => Card2::T,
                    '2' => Card2::Two,
                    '3' => Card2::Three,
                    '4' => Card2::Four,
                    '5' => Card2::Five,
                    '6' => Card2::Six,
                    '7' => Card2::Seven,
                    '8' => Card2::Eight,
                    '9' => Card2::Nine,
                    _ => Card2::A, // handle impossible behaviour
                }
            })
            .collect();

        let mut cards_counts: HashMap<Card2, u32> = HashMap::new();

        for card in &cards_vec {
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

        let mut hand_type_determined: bool = false;

        if cards_counts.values().count() == 1 {
            hand_type = Some(HandType::FiveOfAKind);
        }

        if cards_counts.values().count() == 2 {
            if cards_counts.contains_key(&Card2::J) && cards_counts.get(&Card2::J).unwrap() != &4 {
                hand_type = Some(HandType::FiveOfAKind);
            } else {
                for card_count in cards_counts.values() {
                    if card_count == &4 {
                        hand_type = Some(HandType::FourOfAKind);
                        break;
                    }
                    if card_count == &3 {
                        hand_type = Some(HandType::FullHouse);
                        break;
                    }
                }
            }
        }

        else if cards_counts.values().count() == 3 {
            let mut not_two_pair: bool = false;
            for card_count in cards_counts.values() {
                if card_count > &2 {
                    not_two_pair = true;
                }
            }
            if not_two_pair {
                hand_type = Some(HandType::ThreeOfAKind);
            } else {
                hand_type = Some(HandType::TwoPair);
            }
        }

        if cards_counts.values().count() == 4 {
            if cards_counts.contains_key(&Card2::J) {
                if cards_counts.get(&Card2::J).unwrap() != &2 {
                    // JJT72 
                }
            }
            hand_type = Some(HandType::OnePair);
        }
        
        if cards_counts.values().count() == 5 {
            if cards_counts.contains_key(&Card2::J) {
                hand_type = Some(HandType::OnePair);
            } else {
                hand_type = Some(HandType::HighCard);
            }
        }

        return Self {
            cards: cards_vec,
            bid,
            hand_type: hand_type.unwrap(),
        };
    }
}

fn part_one() {
    let file_path = "day7/input.txt";
    let file = File::open(file_path).unwrap();
    let mut file = BufReader::new(file);

    let mut sum: u64 = 0;

    let start_time = Instant::now();

    let mut hands: Vec<Hand> = Vec::new();

    for line in file.by_ref().lines() {
        let line = line.unwrap();

        let (hand, bid) = line.split_once(" ").unwrap();

        hands.push(Hand::new(hand.to_string(), bid.parse::<u32>().unwrap()))
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
    let file_path = "day7/input.txt";
    let file = File::open(file_path).unwrap();
    let mut file = BufReader::new(file);

    let mut sum: u64 = 0;

    let start_time = Instant::now();

    let mut hands: Vec<Hand> = Vec::new();

    for line in file.by_ref().lines() {
        let line = line.unwrap();
        let (hand, bid) = line.split_once(" ").unwrap();
        hands.push(Hand::new(hand.to_string(), bid.parse::<u32>().unwrap()))
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
    part_one();
}
