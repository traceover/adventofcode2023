use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("inputs/8.txt").expect("Failed to read input file");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn get_steps<'a>(graph: &'a Graph, mut state: &'a str, terminator: &str) -> usize {
    let mut n = 0;
    'outer: loop {
        for mv in &graph.moves {
            match mv {
                Move::Left => state = graph.table[state].0.as_str(),
                Move::Right => state = graph.table[state].1.as_str(),
            }
            n += 1;
            if state.ends_with(terminator) {
                break 'outer;
            }
        }
    }
    n
}

fn part_one(input: &str) -> usize {
    let graph = Graph::parse(input);
    get_steps(&graph, "AAA", "ZZZ")
}

fn part_two(input: &str) -> usize {
    let graph = Graph::parse(input);
    let states = graph.table.keys().filter(|key| key.ends_with('A')).map(String::as_str).collect::<Vec<&str>>();
    
    debug_assert_eq!(
        graph.table.keys().filter(|key| key.ends_with('A')).count(),
        graph.table.keys().filter(|key| key.ends_with('Z')).count(),
    );

    let mut steps = vec![];
    for &state in &states {
        let n = get_steps(&graph, state, "Z");
        steps.push(n);
    }
    lcm(&steps)
}

/// The least common multiple of two numbers is the lowest
/// possible number than can be divisible by both numbers.
pub fn lcm(nums: &[usize]) -> usize {
    nums.iter().fold(1, |acc, &n| n * acc / gcd(n, acc))
}

pub fn gcd<T: Copy + Default + PartialEq + std::ops::Rem<Output = T>>(a: T, b: T) -> T {
    if b == Default::default() {
        return a;
    }

    gcd(b, a % b)
}

#[derive(Debug, Clone)]
pub enum Move {
    Left,
    Right,
}

fn parse_move(x: char) -> Option<Move> {
    match x {
        'L' => Some(Move::Left),
        'R' => Some(Move::Right),
        _ => None,
    }
}

#[derive(Default, Clone)]
pub struct Graph {
    moves: Vec<Move>,
    table: HashMap<String, (String, String)>
}

impl Graph {
    fn parse(s: &str) -> Self {
        let mut table = HashMap::new();
        let lines = s.lines().filter(|line| !line.is_empty()).collect::<Vec<_>>();
        if let Some((first, rest)) = lines.split_first() {
            let moves = first.chars().filter_map(parse_move).collect();
            for (row, line) in rest.iter().enumerate() {
                if let [lhs, rhs] = line.split(" = ").collect::<Vec<_>>()[..] {
                    let key = lhs.to_string();
                    let from = rhs[1..4].to_string();
                    let to = rhs[6..9].to_string();
                    table.insert(key, (from, to));
                } else {
                    panic!("Invalid format for line {}: {}", row + 1, line);
                }
            }
            Self { moves, table }
        } else {
            panic!("Expected a least 1 line in input: `{}`", s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_example_1() {
        let input = concat!(
            "RL\n",
            "AAA = (BBB, CCC)\n",
            "BBB = (DDD, EEE)\n",
            "CCC = (ZZZ, GGG)\n",
            "DDD = (DDD, DDD)\n",
            "EEE = (EEE, EEE)\n",
            "GGG = (GGG, GGG)\n",
            "ZZZ = (ZZZ, ZZZ)\n",
        );
        assert_eq!(part_one(input), 2);
    }

    #[test]
    fn test_example_2() {
        let input = concat!(
            "LLR\n",
            "AAA = (BBB, BBB)\n",
            "BBB = (AAA, ZZZ)\n",
            "ZZZ = (ZZZ, ZZZ)\n",
        );
        assert_eq!(part_one(input), 6);
    }

    #[test]
    fn test_example_3() {
        let input = concat!(
            "LR\n",
            "11A = (11B, XXX)\n",
            "11B = (XXX, 11Z)\n",
            "11Z = (11B, XXX)\n",
            "22A = (22B, XXX)\n",
            "22B = (22C, 22C)\n",
            "22C = (22Z, 22Z)\n",
            "22Z = (22B, 22B)\n",
            "XXX = (XXX, XXX)\n",
        );
        assert_eq!(part_two(input), 6);
    }
}
