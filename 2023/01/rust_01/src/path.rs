use crate::opt::Question;
use crate::opt::Opt;

pub fn get_path(opt: &Opt) -> &'static str {

    match opt.small {
        true => {
            match opt.question {
                Question::A => "../input/01_A_small.in",
                Question::B => "../input/01_B_small.in",
            }
        },
        false => "../input/01.in",
    }
}
