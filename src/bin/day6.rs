use anyhow::{anyhow, Context, Error, Result};
use std::iter;
use std::str::FromStr;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/6.txt").expect("Failed to read input file");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

pub fn part_one(input: &str) -> i64 {
    let input = input.parse::<PartOne>().expect("failed to parse input");

    let mut result = 1;
    for (t, d) in iter::zip(input.times, input.distances) {
        result *= calculate_ways_to_win(t, d);
    }
    result
}

pub fn part_two(input: &str) -> i64 {
    let input = input.parse::<PartTwo>().expect("failed to parse input");
    calculate_ways_to_win(input.time, input.distance)
}

/// The number of ways to achieve a distance greater than `record`.
pub fn calculate_ways_to_win(time: i64, record: i64) -> i64 {
    let mut ways = 0;
    for hold_time in 1..time {
        let travel_time = time - hold_time;
        let distance = hold_time * travel_time;
        if distance > record {
            ways += 1;
        }
    }
    ways
}

struct PartOne {
    times: Vec<i64>,
    distances: Vec<i64>,
}

impl FromStr for PartOne {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        if lines.len() != 2 {
            return Err(anyhow!("input must have exactly 2 lines"));
        }
        let times = parse_numbers(
            lines[0]
                .strip_prefix("Time:")
                .context("expected first line to start with `Time`")?,
        )?;
        let distances = parse_numbers(
            lines[1]
                .strip_prefix("Distance:")
                .context("expected first line to start with `Distance`")?,
        )?;
        Ok(Self { times, distances })
    }
}

/// Splits a string by whitespace and maps each item to a number.
pub fn parse_numbers(s: &str) -> Result<Vec<i64>> {
    s.split_whitespace()
        .map(str::parse::<i64>)
        .collect::<Result<_, _>>()
        .map_err(Error::new)
}

/// The input data for part two.
struct PartTwo {
    time: i64,
    distance: i64,
}

impl FromStr for PartTwo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        if lines.len() != 2 {
            return Err(anyhow!("input must have exactly 2 lines"));
        }
        let time = lines[0]
            .strip_prefix("Time:")
            .context("expected first line to start with `Time`")?
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse()
            .context("failed to parse numbers")?;
        let distance = lines[1]
            .strip_prefix("Distance:")
            .context("expected first line to start with `Distance`")?
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse()
            .context("failed to parse numbers")?;
        Ok(Self { time, distance })
    }
}
