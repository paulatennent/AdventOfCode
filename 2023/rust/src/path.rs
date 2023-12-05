use crate::opt::Opt;
use crate::opt::Question;

pub fn get_path(opt: &Opt) -> String {
    let day = format!("{:0>2}", opt.day.to_string());

    match opt.small {
        true => match opt.question {
            Question::A => format!("../input/{}_A_small.in", day).clone(),
            Question::B => format!("../input/{}_B_small.in", day),
        },
        false => format!("../input/{}.in", day),
    }
}
