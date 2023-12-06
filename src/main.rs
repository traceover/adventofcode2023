pub mod day1;

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    day1::run(&fs::read_to_string("inputs/1.txt")?);
    Ok(())
}
