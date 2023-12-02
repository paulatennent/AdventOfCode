use std::{error::Error, fs};

mod opt;
mod path;
mod days;
use crate::path::get_path;

fn main() -> Result<(), Box<dyn Error>> {
    
    let opt = opt::get_opt();
    let input = fs::read_to_string(get_path(&opt))?;
    match opt.day {
        1 => days::day_01::day_01(opt, input),
        _ => println!("Day {} is unimplemented", opt.day),
    }

    Ok(())
}

