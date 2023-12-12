use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/4.txt").expect("Failed to read input file");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

pub fn part_one(input: &str) -> i64 {
    let cards = parse(input);
    let mut sum = 0;
    for card in cards {
        if card > 0 {
            sum += 1 << (card - 1);
        }
    }
    sum
}

pub fn part_two(input: &str) -> i64 {
    let cards = parse(input);

    fn recursively_add_duplicates(dups: &mut Vec<i64>, cards: &Vec<i64>, cur: usize) {
        for i in cur + 1..=cur + cards[cur] as usize {
            recursively_add_duplicates(dups, cards, i);
            dups.push(i as i64);
        }
    }

    let mut dups = vec![];
    for i in 0..cards.len() {
        recursively_add_duplicates(&mut dups, &cards, i);
    }

    (cards.len() + dups.len()) as i64
}

pub fn parse(input: &str) -> Vec<i64> {
    let mut result = Vec::new();
    for line in input.lines() {
        let line = line.strip_prefix("Card").unwrap();
        let mut parts = line.split(':').map(str::trim);
        let _card_id: u64 = parts.next().unwrap().parse().unwrap();
        let mut card_numbers = parts.next().unwrap().split("|");
        let actual_numbers: Vec<i64> = card_numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let your_numbers: Vec<i64> = card_numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        result.push(
            your_numbers
                .iter()
                .filter(|n| actual_numbers.contains(n))
                .collect::<Vec<_>>()
                .len() as i64,
        );
    }
    result
}
