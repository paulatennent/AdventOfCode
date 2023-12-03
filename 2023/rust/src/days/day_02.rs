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
    input
        .split('\n')
        .filter_map(|line| {
            let re = Regex::new(r"Game ([0-9]*): (.*)").unwrap();
            let Some((_, [game, balls])) = re
                .captures_iter(line)
                .map(|caps| caps.extract())
                .next() else {return None};
            let re = Regex::new(r"[,;] ").unwrap();
            let balls: Vec<&str> = re.split(balls).collect();
            let valid = balls
                .iter()
                .map(|item| {
                    let mut split = item.split(' ');
                    let num: i32 = split.next().unwrap().parse().unwrap();
                    let colour = split.next().unwrap();
                    let max = match colour {
                        "red" => 12,
                        "green" => 13,
                        "blue" => 14,
                        _ => panic!("colour doesnt exist"),
                    };
                    num <= max
                })
                .all(|valid| valid);
            if valid {
                Some(game.parse::<i32>().unwrap())
            } else {
                None
            }
        })
        .sum()
}

fn get_b(input: String) -> i32 {
    input
        .split('\n')
        .filter_map(|line| {
            let re = Regex::new(r"Game [0-9]*: (.*)").unwrap();
            let Some((_, [balls])) = re
                .captures_iter(line)
                .map(|caps| caps.extract())
                .next() else {return None};
            let re = Regex::new(r"[,;] ").unwrap();
            let balls: Vec<&str> = re.split(balls).collect();
            let mut hashmap = [0, 0, 0];
            balls.iter().for_each(|item| {
                let mut split = item.split(' ');
                let num: i32 = split.next().unwrap().parse().unwrap();
                let colour = match split.next().unwrap() {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("colour doesnt exist"),
                };
                hashmap[colour] = hashmap[colour].max(num);
            });
            Some(hashmap[0] * hashmap[1] * hashmap[2])
        })
        .sum()
}
