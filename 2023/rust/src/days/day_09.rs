use crate::opt::{Opt, Question};

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
    let inputs = input
        .split("\n")
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();
    calculate(inputs)
}

fn calculate(inputs: Vec<Vec<i32>>) -> i32 {
    inputs.iter().map(|input| {
        let mut lasts = vec!();
        let mut curr_row = input.clone();
        while !curr_row.iter().all(|num| *num == 0) {
            let last;
            let start = curr_row.remove(0);
            (curr_row, last) = curr_row.iter().fold((vec!(), start), |(mut acc, prev), curr| {
                acc.push(curr - prev);
                (acc, *curr)
            });
            lasts.push(last);
        };
        let res = lasts.iter().sum::<i32>();
        res
    }).sum()
}

fn get_b(input: String) -> i32 {
    let inputs = input
        .split("\n")
        .map(|line| {
            let mut line = line.split(' ')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>();
            line.reverse();
            line
        })
        .collect::<Vec<_>>();
    calculate(inputs)
}
