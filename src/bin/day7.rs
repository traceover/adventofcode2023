use std::fs;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("inputs/7.txt").expect("failed to read input file");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

pub fn part_one(input: &str) -> i32 {
    let mut hands: Vec<_> = input.lines().map(parse_hand_and_bid).collect();
    hands.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut sum = 0;
    for (i, (_, bid)) in hands.iter().enumerate() {
        sum += bid * (i as i32 + 1)
    }
    sum
}

pub fn part_two(_input: &str) -> i64 {
    0
}

fn parse_hand_and_bid(s: &str) -> (Hand, i32) {
    if let [cards, bid] = s.split_whitespace().collect::<Vec<_>>()[..] {
        let cards: Vec<_> = cards
            .chars()
            .map(|x| match x {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '0'..='9' => x as i32 - '0' as i32,
                _ => panic!("Invalid character in hand: {}", x),
            })
            .collect();

        let bid = bid.parse().unwrap();
        let rank = get_rank(&cards);

        (Hand { cards, rank }, bid)
    } else {
        println!("Invalid line: {}", s);
        Default::default()
    }
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RankType {
    #[default]
    HighCard,
    OnePair,
    TwoPair, // High pair, Low pair
    ThreeOfAKind,
    FullHouse, // Three of a kind, Pair
    FourOfAKind,
    FiveOfAKind,
}

pub fn get_rank(hand: &[i32]) -> RankType {
    let mut rank_counts = HashMap::new();

    for &card in hand {
        *rank_counts.entry(card).or_insert(0) += 1;
    }

    let mut pairs = 0;
    let mut three = false;
    let mut four = false;

    for &count in rank_counts.values() {
        match count {
            2 => pairs += 1,
            3 => three = true,
            4 => four = true,
            _ => {}
        }
    }

    if rank_counts.len() == 1 {
        RankType::FiveOfAKind
    } else if four {
        RankType::FourOfAKind
    } else if three {
        if pairs > 0 {
            RankType::FullHouse
        } else {
            RankType::ThreeOfAKind
        }
    } else {
        match pairs {
            2 => RankType::TwoPair,
            1 => RankType::OnePair,
            _ => RankType::HighCard,
        }
    }
}

#[derive(Debug, Default, PartialOrd, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<i32>,
    rank: RankType,
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
            for (&a, &b) in (&self.cards).iter().zip(&other.cards) {
                if a == b {
                    continue;
                }
                return a.cmp(&b);
            }
            Ordering::Equal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::part_one;

    #[test]
    fn test_part_one() {
        let input = concat!(
            "32T3K 765\n",
            "T55J5 684\n",
            "KK677 28\n",
            "KTJJT 220\n",
            "QQQJA 483\n",
        );
        assert_eq!(part_one(input), 6440);
    }
}
