use std::str::Chars;

pub fn run(input: &str) {
    println!("Day 01");
    println!("    Part one: {}", part_one(input));
    println!("    Part two: {}", part_two(input));
}

/// The sum of all of the first and last digits found in each line.
pub fn part_one(input: &str) -> i64 {
    input
        .lines()
        .map(str::chars)
        .map(|chars| chars.filter_map(|c| c.to_digit(10).map(|x| x.into())))
        .map(two_digit_number_from_digits)
        .sum()
}

/// The sum of all of the first and last digits found in each line,
/// where a digit can also be the literal spelling of a number,
/// such as `one`, `two`, or `three`.
pub fn part_two(input: &str) -> i64 {
    input
        .lines()
        .map(Numbers::new)
        .map(two_digit_number_from_digits)
        .sum()
}

fn two_digit_number_from_digits(digits: impl Iterator<Item = i64>) -> i64 {
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
    chars: Chars<'a>,
}

impl<'a> Numbers<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { chars: input.chars() }
    }

    pub fn parse_number(&mut self, first_char: char) -> Option<i64> {
        let mut word = first_char.to_string();
        word.extend(self.chars.by_ref().take_while(|c| c.is_alphabetic()));

        match word.as_str() {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            "zero" => Some(0),
            _ => None,
        }
    }
}

impl<'a> Iterator for Numbers<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.chars.next() {
            if let Some(x) = c.to_digit(10) {
                return Some(x.into());
            } else if c.is_alphabetic() {
                return self.parse_number(c);
            }
            // Continue if it's neither a digit nor a letter
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
