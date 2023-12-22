use crate::opt::{Opt, Question};
use std::cmp::Ordering;
use std::collections::HashMap;

enum Part {
    A,
    B,
}

pub fn solve(opt: Opt, input: String) {
    match opt.question {
        Question::A => {
            println!("Solution to A: {}", get(input, Part::A));
        }
        Question::B => {
            println!("Solution to B: {}", get(input, Part::B));
        }
    }
}

fn get(input: String, part: Part) -> i32 {
    let mut input = input
        .split("\n")
        .map(|both| {
            let mut it = both.split(" ");
            let code = it.next().unwrap();
            let bid: i32 = it.next().unwrap().parse().unwrap();
            let hand = code
                .chars()
                .map(|c| Card::from_char(c, &part))
                .collect::<Vec<_>>();
            let hand_type = HandType::from_vec(&hand);
            (hand_type, hand, bid)
        })
        .collect::<Vec<_>>();

    input.sort_by(overall);

    input.iter().for_each(|val| {
        println!("{:?}", val);
    });

    input
        .iter()
        .enumerate()
        .map(|(i, (_hand_type, _card, bid))| (i as i32 + 1) * bid)
        .sum()
}

fn overall(a: &(HandType, Vec<Card>, i32), b: &(HandType, Vec<Card>, i32)) -> Ordering {
    let (a_hand_type, a_card, _) = a;
    let (b_hand_type, b_card, _) = b;

    if a_hand_type == b_hand_type {
        a_card.cmp(&b_card)
    } else {
        a_hand_type.cmp(&b_hand_type)
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl HandType {
    fn from_vec(hand: &Vec<Card>) -> Self {
        let mut hand_map: HashMap<Card, i32> = HashMap::new();
        hand.iter().for_each(|card| {
            hand_map
                .entry(*card)
                .and_modify(|card| *card += 1)
                .or_insert(1);
        });

        let n_jokers = hand_map.remove(&Card::Joker).unwrap_or(0);
        let handtype = if Self::is_five_of_kind(&hand_map, n_jokers) {
            HandType::FiveOfKind
        } else if Self::is_four_of_kind(&hand_map, n_jokers) {
            HandType::FourOfKind
        } else if Self::is_full_house(&hand_map) {
            HandType::FullHouse
        } else if Self::is_three_of_kind(&hand_map, n_jokers) {
            HandType::ThreeOfKind
        } else if Self::is_two_pair(&hand_map) {
            HandType::TwoPair
        } else if Self::is_one_pair(&hand_map, n_jokers) {
            HandType::OnePair
        } else {
            HandType::HighCard
        };
        handtype
    }

    fn is_five_of_kind(hand: &HashMap<Card, i32>, n_jokers: i32) -> bool {
        hand.iter().any(|(_k, v)| (v + n_jokers) == 5)
    }

    fn is_four_of_kind(hand: &HashMap<Card, i32>, n_jokers: i32) -> bool {
        hand.iter().any(|(_k, v)| (v + n_jokers) == 4)
    }

    fn is_full_house(hand: &HashMap<Card, i32>) -> bool {
        hand.iter().all(|(_k, v)| v >= &2)
    }

    fn is_three_of_kind(hand: &HashMap<Card, i32>, n_jokers: i32) -> bool {
        hand.iter().any(|(_k, v)| (v + n_jokers) == 3)
    }

    fn is_two_pair(hand: &HashMap<Card, i32>) -> bool {
        hand.iter()
            .filter(|(_k, v)| **v == 2)
            .collect::<Vec<_>>()
            .len()
            == 2
    }

    fn is_one_pair(hand: &HashMap<Card, i32>, n_jokers: i32) -> bool {
        hand.iter().any(|(_k, v)| (v + n_jokers) == 2)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Joker,
    Num(u32), // 2-9
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(s: char, part: &Part) -> Self {
        match s {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => match part {
                Part::A => Card::J,
                Part::B => Card::Joker,
            },
            'T' => Card::T,
            num => Card::Num(char::to_digit(num, 10).unwrap()),
        }
    }
}
