use structopt::StructOpt;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
pub enum Question {
    A,
    B,
}

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(short)]
    pub small: bool,

    #[structopt(short, default_value = "A")]
    pub question: Question,

    #[structopt(name = "DAY")]
    pub day: i32,
}

pub fn get_opt() -> Opt {
    Opt::from_args()
}
