use crate::opt::Opt;
use crate::opt::Question;

pub fn get_path(opt: &Opt) -> String {
    let day = match opt.day {
        1 => "01",
        2 => "02",
        3 => "03",
        _ => panic!("Provided day not implemented"),
    };

    match opt.small {
        true => match opt.question {
            Question::A => format!("../input/{}_A_small.in", day).clone(),
            Question::B => format!("../input/{}_B_small.in", day),
        },
        false => format!("../input/{}.in", day),
    }
}
