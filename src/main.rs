pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use anyhow::Result;
use std::fs;
use std::thread;

type Challenge = (fn(&str) -> i64, fn(&str) -> i64);

fn main() -> Result<()> {
    let challenges: Vec<Challenge> = vec![
        (day1::part_one, day1::part_two),
        (day2::part_one, day2::part_two),
        (day3::part_one, day3::part_two),
        (day4::part_one, day4::part_two),
        (day5::part_one, day5::part_two),
        (day6::part_one, day6::part_two),
    ];

    let mut handles = vec![];
    for (i, (part_one, part_two)) in challenges.into_iter().enumerate() {
        let handle = thread::spawn(move || {
            let input = fs::read_to_string(format!("inputs/{}.txt", i + 1))
                .expect(format!("Failed to read input for day {}", i + 1).as_str());
            let result_one = part_one(&input);
            let result_two = part_two(&input);
            (result_one, result_two)
        });
        handles.push(handle);
    }

    for (i, handle) in handles.into_iter().enumerate() {
        let (part_one, part_two) = handle.join().expect("Thread panicked");
        println!("Day {:02}", i + 1);
        println!("    Part one: {}", part_one);
        println!("    Part two: {}", part_two);
    }

    Ok(())
}
