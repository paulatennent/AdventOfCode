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

fn get_a(_input: String) -> i64 {
    todo!();
}

fn get_b(_input: String) -> i64 {
    todo!();
}
