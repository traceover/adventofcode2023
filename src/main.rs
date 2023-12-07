pub mod inputs;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use std::io::{self, Write};
use std::fs::{self, File};
use std::path::Path;
use std::thread::{self, JoinHandle};
use anyhow::{Context, Result, Error};

type Challenge = (fn(&str) -> i64, fn(&str) -> i64);

pub const CURRENT_DAY: usize = 6;

fn main() -> Result<()> {
    let input_dir = inputs::DEFAULT_INPUT_PATH;

    if !Path::new(&input_dir).exists() {
        fs::create_dir_all(&input_dir)
            .with_context(|| format!("Could not create inputs directory: {}", input_dir))?;
        
        let mut cookie = String::new();
        println!("Please enter your session cookie:");
        io::stdin().read_line(&mut cookie).context("Could not read cookie")?;

        for day in 1..=CURRENT_DAY {
            let file_path = format!("{}/{}.txt", input_dir, day);
            let mut file = File::create(&file_path)
                .with_context(|| format!("Could not create file: {}", file_path))?;
            let input = inputs::download_input(day, "2023", &cookie)?;
            file.write_all(input.as_bytes()).context("Could not write file")?;
        }
    }

    let challenges: Vec<Challenge> = vec![
        (day1::part_one, day1::part_two),
        (day2::part_one, day2::part_two),
        (day3::part_one, day3::part_two),
        (day4::part_one, day4::part_two),
        (day5::part_one, day5::part_two),
        (day6::part_one, day6::part_two),
    ];

    let mut handles: Vec<JoinHandle<Result<(i64, i64), Error>>> = vec![];
    for (i, (part_one, part_two)) in challenges.into_iter().enumerate() {
        let handle = thread::spawn(move || {
            let input = fs::read_to_string(format!("inputs/{}.txt", i + 1))
                .with_context(|| format!("Failed to read input for day {}", i + 1))?;
            let result_one = part_one(&input);
            let result_two = part_two(&input);
            Ok((result_one, result_two))
        });
        handles.push(handle);
    }

    for (i, handle) in handles.into_iter().enumerate() {
        let (part_one, part_two) = handle.join().expect("Thread panicked")?;
        println!("Day {:02}", i + 1);
        println!("    Part one: {}", part_one);
        println!("    Part two: {}", part_two);
    }

    Ok(())
}
