pub mod day1;
pub mod day6;

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    day1::run(&fs::read_to_string("inputs/1.txt")?);
    day6::run(&fs::read_to_string("inputs/6.txt")?);
    Ok(())
}
