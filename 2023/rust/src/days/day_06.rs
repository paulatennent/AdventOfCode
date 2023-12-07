use crate::opt::{Opt, Question};
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::ops::Range;

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

fn get_a(input: String) -> i64 {
    let mut input = input.split('\n');
    let re = Regex::new(r" +").unwrap();
    let time = re.split(input.next().unwrap());
    let dist = re.split(input.next().unwrap());

    let time = time.map(|num| {
        num.parse::<i32>();
    })
    todo!();
}

fn get_b(input: String) -> i64 {
    todo!();
}
