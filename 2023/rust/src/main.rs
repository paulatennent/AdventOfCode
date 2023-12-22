use std::{error::Error, fs};

mod days;
mod opt;
mod path;
use crate::path::get_path;

fn main() -> Result<(), Box<dyn Error>> {
    let opt = opt::get_opt();
    let path = get_path(&opt);
    println!("Reading from file {}", path);
    let input = fs::read_to_string(path)?;
    match opt.day {
        1 => days::day_01::solve(opt, input),
        2 => days::day_02::solve(opt, input),
        3 => days::day_03::solve(opt, input),
        4 => days::day_04::solve(opt, input),
        5 => days::day_05::solve(opt, input),
        6 => days::day_06::solve(opt, input),
        7 => days::day_07::solve(opt, input),
        8 => days::day_08::solve(opt, input),
        9 => days::day_09::solve(opt, input),
        _ => println!("Day {} is unimplemented", opt.day),
    }

    Ok(())
}
