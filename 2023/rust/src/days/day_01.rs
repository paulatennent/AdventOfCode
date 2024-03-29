use crate::opt::{Opt, Question};
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

fn get_a(input: String) -> u32 {
    input
        .split('\n')
        .map(|s| s.to_string())
        .filter_map(|s| {
            let digits = s.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
            let len = digits.len();
            match len {
                0 => None,
                _ => Some(digits[0] * 10 + digits[len - 1]),
            }
        })
        .sum()
}

fn get_b(input: String) -> i32 {
    let digit_names = HashMap::from([
        (0, "zero"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ]);

    input
        .split('\n')
        .filter_map(|s| {
            let first = find_first(s.to_string(), &digit_names);
            let last = find_last(s.to_string(), &digit_names);
            match (first, last) {
                (Some(first), Some(last)) => Some(first * 10 + last),
                _ => None,
            }
        })
        .sum()
}

fn find_first(s: String, digit_names: &HashMap<i32, &str>) -> Option<i32> {
    let digits = (1..10).collect::<Vec<_>>();

    let mut words_occ = digits
        .iter()
        .map(|d| (s.find(digit_names.get(&d).unwrap()), d))
        .collect::<Vec<_>>();

    let mut digits_occ = digits
        .iter()
        .map(|d| (s.find(&d.to_string()), d))
        .collect::<Vec<_>>();

    words_occ.append(&mut digits_occ);

    words_occ
        .iter()
        .filter_map(|(index, num)| match index {
            None => None,
            Some(index) => Some((index, num)),
        })
        .min() // find min index
        .map(|(_index, num)| **num as i32)
}

fn find_last(s: String, digit_names: &HashMap<i32, &str>) -> Option<i32> {
    let digits = (0..10).collect::<Vec<_>>();

    let mut words_occ = digits
        .iter()
        .map(|d| {
            let index = s.rfind(digit_names.get(&d).unwrap());
            (index, d)
        })
        .collect::<Vec<_>>();

    let mut digits_occ = digits
        .iter()
        .map(|d| {
            let index = s.rfind(&d.to_string());
            (index, d)
        })
        .collect::<Vec<_>>();

    words_occ.append(&mut digits_occ);

    words_occ
        .iter()
        .filter_map(|(index, number)| match index {
            None => None,
            Some(index) => Some((index, number)),
        })
        .max()
        .map(|(_index, num)| **num as i32)
}
