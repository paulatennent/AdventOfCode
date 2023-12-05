use crate::opt::{Opt, Question};
use regex::Regex;
use std::collections::{HashMap, HashSet};

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
    let row_length = input.find('\n').unwrap() as i32 + 1;

    // a vec of iterators that holds 1 iterator for each location surrounding
    // the currently looked at value. (8 total)
    let mut iters = vec![];

    // add in the first 4
    // _ _ _
    // _ c I
    // I I I
    for i in 0..row_length + 1 {
        if i == 0 || i == 1 || i == 2 || i == row_length {
            let iter = input.chars().peekable();
            iters.push(iter);
        }
        iters.iter_mut().for_each(|iter| {
            iter.next();
        });
    }

    let iter_main = input.chars().enumerate().peekable();

    let mut sum = 0;
    let mut num = 0;
    let mut is_part = false;
    for (i, c) in iter_main {
        let i = i as i32;

        // add in the remaining 4
        // I I I
        // I c i
        // i i i
        if i == 1 || i == row_length - 1 || i == row_length || i == row_length + 1 {
            let iter = input.chars().peekable();
            iters.push(iter);
        }

        match c {
            n if n.is_ascii_digit() => {
                num = num * 10 + n.to_digit(10).unwrap();

                let symbol_found = iters
                    .iter_mut()
                    .filter_map(|iter| iter.peek().map(|val| is_symbol(val)))
                    .any(|symbol| symbol);
                if symbol_found {
                    is_part = true;
                }
            }
            _ => {
                if is_part {
                    sum = sum + num;
                }
                num = 0;
                is_part = false;
            }
        }

        iters.iter_mut().for_each(|iter| {
            iter.next();
        });
    }
    sum
}

fn is_symbol(c: &char) -> bool {
    !(c.is_ascii_digit() || *c == '.' || *c == '\n')
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Info {
    start: usize,
    num: i32,
}

fn get_b(input: String) -> i32 {
    let mut nums: HashMap<i32, Info> = HashMap::new();

    // hash each number, location -> (start location, num)
    let re = Regex::new(r"([0-9]+)").unwrap();
    re.find_iter(&input).for_each(|cap| {
        let num: i32 = cap.as_str().parse().unwrap();
        (cap.start()..cap.end()).for_each(|loc| {
            nums.insert(
                loc as i32,
                Info {
                    start: cap.start(),
                    num,
                },
            );
        })
    });

    // this is to iterate over surrounding values
    let row_len = input.find('\n').unwrap() as i32 + 1;
    let surround: Vec<i32> = vec![
        -row_len - 1,
        -row_len,
        -row_len + 1,
        -1,
        1,
        row_len - 1,
        row_len,
        row_len + 1,
    ];

    // find each *, find surrounding numbers, product then sum!
    let re = Regex::new(r"(\*)").unwrap();
    re.find_iter(&input)
        .filter_map(|cap| {
            let gear_loc = cap.start() as i32;
            let surround_nums = surround
                .iter()
                .filter_map(|offset| nums.get(&(gear_loc + offset)))
                .collect::<HashSet<_>>();
            if surround_nums.len() == 2 {
                Some(surround_nums.iter().map(|info| info.num).product::<i32>())
            } else {
                None
            }
        })
        .sum()
}
