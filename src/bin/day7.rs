use std::fs;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("inputs/7.txt").expect("failed to read input file");
    println!("Part one: {}", run(&input, false));
    println!("Part two: {}", run(&input, true));
}

pub fn run(input: &str, is_v2: bool) -> i32 {
    let mut hands: Vec<Hand> = input.lines().map(|line| parse_hand(line, is_v2)).collect();
    hands.sort(|a, b| a.cmp(&b)); // Check out `impl Ord for Hand`

    /*for hand in &hands {
        println!("{} => {:?} => {}", hand.cards, hand.rank, hand.bid);
    }*/

    let mut sum = 0;
    for (i, Hand { bid, .. }) in hands.iter().enumerate() {
        sum += bid * (i as i32 + 1)
    }
    sum
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RankType {
    #[default]
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn get_rank(hand: &str) -> RankType {
    let mut rank_counts = HashMap::new();

    for card in hand.chars() {
        *rank_counts.entry(card).or_insert(0) += 1;
    }

    let max_rank_count = rank_counts.values().max().unwrap_or(&0);
    let pairs = rank_counts.values().filter(|&&c| c == 2).count();

    match max_rank_count {
        5 => RankType::FiveOfAKind,
        4 => RankType::FourOfAKind,
        3 if pairs > 0 => RankType::FullHouse,
        3 => RankType::ThreeOfAKind,
        _ => match pairs {
            2 => RankType::TwoPair,
            1 => RankType::OnePair,
            _ => RankType::HighCard,
        },
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd)]
pub struct Hand {
    cards: String,
    rank: RankType,
    bid: i32,
}

impl Hand {
    fn new(cards: String, rank: RankType, bid: i32) -> Self {
        Hand { cards, rank, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cards == other.cards {
            Ordering::Equal
        } else if self.rank > other.rank {
            Ordering::Greater
        } else if self.rank < other.rank {
            Ordering::Less
        } else {
            for (a, b) in self.cards.chars().map(card_value).zip(other.cards.chars().map(card_value)) {
                if a == b {
                    continue;
                }
                return a.cmp(&b);
            }
            Ordering::Equal
        }
    }
}

fn apply_wildcards(s: &str) -> String {
    if s == "JJJJJ" {
        return "KKKKK".to_string();
    }
    let wildcard_count = s.chars().filter(|&c| c == 'J').count();
    if wildcard_count > 0 {
        let max_card = most_significant_card(s);
        s.replace('J', &max_card.to_string())
    } else {
        s.to_string()
    }
}

fn parse_hand(s: &str, is_v2: bool) -> Hand {
    if let [cards, bid] = s.split_whitespace().collect::<Vec<_>>()[..] {
        let bid = bid.parse().unwrap();

        let rank = {
            if is_v2 {
                get_rank(&apply_wildcards(cards))
            } else {
                get_rank(cards)
            }
        };

        let cards_for_scoring = if is_v2 {
            cards.replace('J', "1")
        } else {
            cards.to_string()
        };

        Hand::new(cards_for_scoring, rank, bid)
    } else {
        println!("Invalid line: {}", s);
        Default::default()
    }
}

pub fn card_value(x: char) -> i32 {
    match x {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '0'..='9' => x as i32 - '0' as i32,
        _ => 0,
    }
}

/// Returns the character that appears most in a string,
/// or if there is a tie, the one with the highest card value.
pub fn most_significant_card(s: &str) -> char {
    let mut counts = HashMap::new();
    for ch in s.chars() {
        if ch != 'J' {
            *counts.entry(ch).or_insert(0) += 1;
        }
    }

    let mut max_char = '\0';
    let mut max_count = 0;
    let mut max_value = 0;

    for (&ch, &count) in &counts {
        let value = card_value(ch);
        if count > max_count || (count == max_count && value > max_value) {
            max_count = count;
            max_value = value;
            max_char = ch;
        }
    }

    max_char
}

#[cfg(test)]
mod tests {
    use super::{run, parse_hand, RankType};
    
    const TEST_INPUT: &str = concat!(
        "32T3K 765\n",
        "T55J5 684\n",
        "KK677 28\n",
        "KTJJT 220\n",
        "QQQJA 483\n",
    );

    #[test]
    fn test_ranking_v1() {
        assert_eq!(parse_hand("J2345 0", false).rank, RankType::HighCard);
        assert_eq!(parse_hand("JJ234 0", false).rank, RankType::OnePair);
        assert_eq!(parse_hand("JJ224 0", false).rank, RankType::TwoPair);
        assert_eq!(parse_hand("J3222 0", false).rank, RankType::ThreeOfAKind);
        assert_eq!(parse_hand("JJ222 0", false).rank, RankType::FullHouse);
        assert_eq!(parse_hand("J2222 0", false).rank, RankType::FourOfAKind);
        assert_eq!(parse_hand("22222 0", false).rank, RankType::FiveOfAKind);
    }

    #[test]
    fn test_ranking_v2() {
        assert_eq!(parse_hand("K2345 0", true).rank, RankType::HighCard);
        assert_eq!(parse_hand("KK234 0", true).rank, RankType::OnePair);
        assert_eq!(parse_hand("KK224 0", true).rank, RankType::TwoPair);
        assert_eq!(parse_hand("K3222 0", true).rank, RankType::ThreeOfAKind);
        assert_eq!(parse_hand("KK222 0", true).rank, RankType::FullHouse);
        assert_eq!(parse_hand("K2222 0", true).rank, RankType::FourOfAKind);
        assert_eq!(parse_hand("22222 0", true).rank, RankType::FiveOfAKind);
    }

    #[test]
    fn test_ranking_with_wild_v2() {
        assert_eq!(parse_hand("23456 0", true).rank, RankType::HighCard);
        assert_eq!(parse_hand("J2345 0", true).rank, RankType::OnePair);
        assert_eq!(parse_hand("J4545 0", true).rank, RankType::FullHouse);
        assert_eq!(parse_hand("J2355 0", true).rank, RankType::ThreeOfAKind);
        assert_eq!(parse_hand("J2555 0", true).rank, RankType::FourOfAKind);
        assert_eq!(parse_hand("J5555 0", true).rank, RankType::FiveOfAKind);
        assert_eq!(parse_hand("JJ345 0", true).rank, RankType::ThreeOfAKind);
        assert_eq!(parse_hand("JJ455 0", true).rank, RankType::FourOfAKind);
        assert_eq!(parse_hand("JJ555 0", true).rank, RankType::FiveOfAKind);
    }

    #[test]
    fn test_example_ranking_v1() {
        let mut ranks = vec![];
        for line in TEST_INPUT.lines() {
            let hand = parse_hand(line, false);
            ranks.push(hand.rank);
        }
        assert_eq!(ranks, vec![
            RankType::OnePair,
            RankType::ThreeOfAKind,
            RankType::TwoPair,
            RankType::TwoPair,
            RankType::ThreeOfAKind,
        ]);
    }

    #[test]
    fn test_example_ranking_v2() {
        let mut ranks = vec![];
        for line in TEST_INPUT.lines() {
            let hand = parse_hand(line, true);
            ranks.push(hand.rank);
        }
        assert_eq!(ranks, vec![
            RankType::OnePair,
            RankType::FourOfAKind,
            RankType::TwoPair,
            RankType::FourOfAKind,
            RankType::FourOfAKind,
        ]);
    }

    #[test]
    fn test_v1() {
        assert_eq!(run(TEST_INPUT, false), 6440);
    }

    #[test]
    fn test_v2() {
        assert_eq!(run(TEST_INPUT, true), 5905);
    }
}
