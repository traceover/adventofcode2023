/// # Day One: Collecting digits from a string.
///
/// My approach for day one was to write a custom iterator over
/// a string that returns only the digits, or, for part two, a
/// sequence of characters like "one" or "two" are transformed
/// into their corresponding numerical value.
///
/// ## Examples
///
/// Basic usage:
///
/// ```
/// let s = "12three4";
/// let numbers: Vec<i64> = s.numbers().collect();
/// assert_eq!(numbers, vec![1, 2, 3, 4]);
/// ```

use std::cmp;

pub fn run(input: &str) {
    println!("Day 01");
    println!("    Part one: {}", part_one(input));
    println!("    Part two: {}", part_two(input));
}

/// The sum of all of the first and last digits found in each line.
pub fn part_one(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        sum += as_two_digit_number(
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|x| x.into())),
        );
    }
    sum
}

/// The sum of all of the first and last digits found in each line,
/// where a digit can also be the literal spelling of a number,
/// such as `one`, `two`, or `three`.
pub fn part_two(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        sum += as_two_digit_number(line.numbers());
    }
    sum
}

/// Maps an iterator of digits into just the first and last digits
/// as a two digit number. The stream `[1, 2, 3]` will become `13` and the
/// stream `[1]` will become `11`.
pub fn as_two_digit_number(digits: impl Iterator<Item = i64>) -> i64 {
    let digits: Vec<_> = digits.collect();
    match digits.as_slice() {
        [first, _rest @ .., last] => first * 10 + last, // Take the first and last digits
        [single] => single * 11,                        // If only one digit, duplicate it
        _ => 0,                                         // Return 0 if no digits are found
    }
}

/// Returns an iterator of the numbers in a string, where
/// only the literal spelling of a number or a digit are allowed.
///
/// Example: "12_ABC_one7" will return `1, 2, 1, 7`
pub struct Numbers<'a> {
    // chars: Chars<'a>,
    // buffer: String,
    input: &'a str,
    pos: usize,
    number_words: Vec<(&'static str, i64)>,
}

impl<'a> Numbers<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            number_words: vec![
                ("zero", 0), ("one", 1), ("two", 2), ("three", 3),
                ("four", 4), ("five", 5), ("six", 6), ("seven", 7),
                ("eight", 8), ("nine", 9)
            ],
        }
    }

    pub fn parse_spelled_number(&mut self) -> Option<i64> {
        let mut longest_match = (0, None); // (length of match, number)
        
        for &(word, number) in &self.number_words {
            if self.pos + word.len() <= self.input.len() && self.input[self.pos..].starts_with(word) {
                longest_match = cmp::max(longest_match, (word.len(), Some(number)));
            }
        }

        if let (len, Some(number)) = longest_match {
            self.pos += len - 1; // -1 so that overlapping "threeight" returns [3, 8]
            return Some(number);
        }

        None
    }
}

impl<'a> Iterator for Numbers<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.input.len() {
            if let Some(digit) = self.input[self.pos..].chars().next().filter(|c| c.is_digit(10)) {
                self.pos += 1; // Advance position after finding a digit
                return Some(digit.to_digit(10).unwrap() as i64);
            }

            if let Some(number) = self.parse_spelled_number() {
                return Some(number);
            }

            self.pos += 1;
        }
        
        None
    }
}

trait NumberIterExt {
    fn numbers(&self) -> Numbers;
}

impl NumberIterExt for str {
    fn numbers(&self) -> Numbers {
        Numbers::new(self)
    }
}

#[cfg(test)]
pub mod tests {
    use super::NumberIterExt;

    /// The example given for part one from `https://adventofcode.com/2023/day/1`.
    #[test]
    pub fn example_part_one() {
        let input = concat!(
            "1abc2\n",
            "pqr3stu8vwx\n",
            "a1b2c3d4e5f\n",
            "treb7uchet\n",
        );
        let val = super::part_one(input);
        assert_eq!(val, 142);
    }

    /// The example given for part two from `https://adventofcode.com/2023/day/1#part2`.
    #[test]
    fn example_part_two() {
        let input = concat!(
            "two1nine\n",
            "eightwothree\n",
            "abcone2threexyz\n",
            "xtwone3four\n",
            "4nineeightseven2\n",
            "zoneight234\n",
            "7pqrstsixteen\n",
        );
        let val = super::part_two(input);
        assert_eq!(val, 281);
    }

    #[test]
    fn numbers() {
        let s = "12three4";
        let numbers: Vec<i64> = s.numbers().collect();
        assert_eq!(numbers, vec![1, 2, 3, 4]);
    }

    #[test]
    fn numbers_overlapping() {
        let s = "threeight1";
        let numbers: Vec<i64> = s.numbers().collect();
        assert_eq!(numbers, vec![3, 8, 1]);
    }
}
