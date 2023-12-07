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
    let (seeds, input) = input.split_once("\n\n").unwrap();

    let seeds: Vec<i64> = seeds
        .split(" ")
        .filter_map(|num| num.parse().ok())
        .collect::<Vec<_>>();

    let mapping: Vec<Vec<(Range<i64>, i64)>> = input
        .split("\n\n")
        .map(|block| {
            let re = Regex::new(r"[0-9]+[0-9 ]*").unwrap();
            re.find_iter(block)
                .map(|line| {
                    let buff = line
                        .as_str()
                        .split(" ")
                        .map(|num| num.parse().expect("should be i32 from regex"))
                        .collect::<Vec<i64>>();
                    let (low, high) = (buff[1], buff[1] + buff[2]);
                    let offset = buff[0] - buff[1];
                    ((low..high), offset)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    seeds
        .iter()
        .map(|seed| {
            let mut seed_acc: i64 = *seed;
            mapping.iter().for_each(|mapping| {
                seed_acc = get_next_mapping(mapping, &seed_acc);
            });
            seed_acc
        })
        .min()
        .unwrap() as i64
}

fn get_next_mapping(mapping: &Vec<(Range<i64>, i64)>, num: &i64) -> i64 {
    let mut valid_mappings = mapping
        .iter()
        .filter_map(|(range, offset)| {
            if range.contains(&num) {
                Some(offset)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if let Some(mapping) = valid_mappings.pop() {
        num + mapping
    } else {
        *num
    }
}

fn get_b(input: String) -> i64 {
    let (seeds, input) = input.split_once("\n\n").unwrap();

    let re = Regex::new(r"(([0-9]+ [0-9]+))").unwrap();
    let seeds: Vec<Range<i64>> = re
        .find_iter(seeds)
        .map(|cap| {
            let range = cap
                .as_str()
                .split(" ")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<_>>();
            range[0]..(range[0] + range[1])
        })
        .collect();

    let mut mapping: Vec<Vec<(i64, i64, i64)>> = input
        .split("\n\n")
        .map(|block| {
            let re = Regex::new(r"[0-9]+[0-9 ]*").unwrap();
            let mut blocks = re
                .find_iter(block)
                .map(|line| {
                    let buff = line
                        .as_str()
                        .split(" ")
                        .map(|num| num.parse().expect("should be i32 from regex"))
                        .collect::<Vec<i64>>();

                    (buff[0], buff[1], buff[2])
                })
                .collect::<Vec<_>>();
            blocks.sort();
            blocks
        })
        .collect::<Vec<_>>();
    mapping.reverse();

    println!("{:?}", mapping);

    let ordering = get_ordering(mapping);
    todo!();
}

fn get_ordering(mapping: Vec<Vec<(i64, i64, i64)>>) {
    let ordering: Vec<(i64, i64)> = Vec::new();
    mapping.iter().for_each(|block| {
        let prev_partitions = ordering
            .iter()
            .map(|(s, r)| vec![*s, *s + *r])
            .flatten()
            .collect::<HashSet<i64>>();
        let curr_partitions = block
            .iter()
            .map(|(d, s, r)| vec![*d, *d + *r])
            .flatten()
            .collect::<HashSet<i64>>();
        let mut both : Vec<_> = prev_partitions
            .union(&curr_partitions).collect();

        both.sort();

        
        println!("{:?}", both);
    });
    todo!();
}
