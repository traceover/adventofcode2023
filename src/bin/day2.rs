use anyhow::{anyhow, Context, Error, Result};
use std::str::FromStr;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/2.txt").expect("Failed to read input file");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

/// The sum of the ids of all possible games.
pub fn part_one(input: &str) -> i64 {
    let games = parse(input).expect("Failed to parse list of games");
    let conf = Set::new(12, 13, 14);
    let mut sum = 0;
    'next_game: for game in games {
        for set in game.sets {
            if !set.is_possible(&conf) {
                continue 'next_game;
            }
        }
        sum += game.id;
    }
    sum
}

pub fn part_two(input: &str) -> i64 {
    let games = parse(input).expect("Failed to parse list of games");
    games.iter().map(Game::power).sum()
}

pub fn parse(input: &str) -> Result<Vec<Game>> {
    input.lines().map(str::parse).collect()
}

#[derive(Default)]
pub struct Set {
    red: i64,
    green: i64,
    blue: i64,
}

impl Set {
    fn new(red: i64, green: i64, blue: i64) -> Self {
        Self { red, green, blue }
    }

    /// Returns true if the set is possible with the provided configuration.
    pub fn is_possible(&self, conf: &Set) -> bool {
        self.red <= conf.red && self.blue <= conf.blue && self.green <= conf.green
    }
}

impl FromStr for Set {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<Vec<&str>> = s
            .split(",")
            .map(|s| s.trim().split_whitespace().collect())
            .collect();

        let mut result: Self = Default::default();

        for item in items {
            if item.len() != 2 {
                return Err(anyhow!("expected 2 items: a number and then a color"));
            }
            let amount = item[0].parse().context("failed to parse cube amount")?;
            match item[1] {
                "red" => result.red = amount,
                "green" => result.green = amount,
                "blue" => result.blue = amount,
                _ => return Err(anyhow!("expected `red`, `green`, or `blue`")),
            }
        }

        Ok(result)
    }
}

pub struct Game {
    id: i64,
    sets: Vec<Set>,
}

impl Game {
    /// The power of a game is the product of the maximum number
    /// of a particular colored cube that was revealed in the game.
    pub fn power(&self) -> i64 {
        let red = self
            .sets
            .iter()
            .map(|set| set.red)
            .max()
            .expect("expected at least one set");
        let green = self
            .sets
            .iter()
            .map(|set| set.green)
            .max()
            .expect("expected at least one set");
        let blue = self
            .sets
            .iter()
            .map(|set| set.blue)
            .max()
            .expect("expected at least one set");
        red * green * blue
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().splitn(2, ": ").collect();
        if parts.len() != 2 {
            return Err(anyhow!(format!(
                "line split by ':' should have exactly 2 parts, found {}",
                parts.len()
            )));
        }

        let id = parts[0]
            .trim_start_matches("Game ")
            .parse()
            .context("failed to parse game id")?;

        let sets = parts[1]
            .split("; ")
            .filter_map(|s| s.parse::<Set>().ok())
            .collect();

        Ok(Game { id, sets })
    }
}
