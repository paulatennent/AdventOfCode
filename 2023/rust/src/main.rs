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
        1 => days::day_01::day_01(opt, input),
        2 => days::day_02::day_02(opt, input),
        _ => println!("Day {} is unimplemented", opt.day),
    }

    Ok(())
}
