use crate::opt::{Opt, Question};
use regex::Regex;
use std::collections::HashMap;

pub fn solve(opt: Opt, input: String) {
    match opt.question {
        Question::A => {
            println!("Solution to A: {}", get_a(input));
        }
        Question::B => {
            println!("Solution to B: {}", get_b(input));
        }
    }
}

fn get_a(input: String) -> i32 {
    let re = Regex::new(r"Card .*: +([0-9 ]+) +\| +([0-9 ]+)").unwrap();
    re.captures_iter(&input)
        .map(|cap| {
            let [winning, nums] = [cap.get(1), cap.get(2)].map(|nums| {
                nums.unwrap()
                    .as_str()
                    .split(" ")
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect::<Vec<_>>()
            });
            winning
                .iter()
                .filter(|winning| nums.contains(&winning))
                .collect::<Vec<_>>()
                .len() as i32
        })
        .filter(|n_winning| n_winning != &0)
        .map(|n_winning| 2_i32.pow((n_winning - 1) as u32))
        .sum::<i32>()
}

fn get_b(input: String) -> i32 {
    let mut cards: HashMap<usize, i32> = HashMap::new();

    let n_lines = input.split('\n').collect::<Vec<_>>().len();

    (0..n_lines).for_each(|i| {
        cards.insert(i, 1);
    });

    let re = Regex::new(r"Card .*: +([0-9 ]+) +\| +([0-9 ]+)").unwrap();
    re.captures_iter(&input).enumerate().for_each(|(i, cap)| {
        let [winning, nums] = [cap.get(1), cap.get(2)].map(|nums| {
            nums.unwrap()
                .as_str()
                .split(" ")
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<_>>()
        });
        let n_winning = winning
            .iter()
            .filter(|winning| nums.contains(&winning))
            .collect::<Vec<_>>()
            .len();
        let n_curr_cards = cards.get(&i).unwrap().clone();
        ((i + 1)..(i + 1 + n_winning)).for_each(|i| {
            if let Some(n_cards) = cards.get_mut(&i) {
                *n_cards = *n_cards + n_curr_cards;
            }
        });
    });
    cards.into_values().sum()
}
