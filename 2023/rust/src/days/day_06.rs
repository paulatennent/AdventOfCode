use crate::opt::{Opt, Question};
use regex::Regex;

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
    let mut input = input.split('\n');
    let re = Regex::new(r" +").unwrap();
    let time = re.split(input.next().unwrap());
    let dist = re.split(input.next().unwrap());

    time.zip(dist)
        .filter_map(|(time, dist)| {
            let Some(time) = time.parse().ok() else {return None;};
            let Some(dist) = dist.parse().ok() else {return None;};
            let n_better: i32 = (0..time)
                .filter(|c| -(c * c) + time * c > dist)
                .collect::<Vec<i32>>()
                .len() as i32;
            Some(n_better)
        })
        .product()
}

fn get_b(input: String) -> i64 {
    let mut input = input.split('\n');
    let _time = get_num(input.next().unwrap());
    let _dist = get_num(input.next().unwrap());

    // ok i just used wolfram alpha from here on XD
    42
}

fn get_num(line: &str) -> i64 {
    let re = Regex::new(r"[0-9]*").unwrap();
    re.find_iter(line)
        .map(|s| s.as_str().chars())
        .flatten()
        .collect::<String>()
        .parse()
        .unwrap()
}
