use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/9.txt").expect("Failed to read input file");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &str) -> i64 {
    let sequences: Vec<_> = input.lines().filter_map(|line| {
        Some(line.split_whitespace().map(|s| s.parse::<i64>().ok()).collect::<Option<Vec<_>>>()?)
    }).collect();

    let mut sum = 0;

    for sequence in sequences {
        let mut diffs = vec![sequence];
        loop {
            let diff: Vec<_> = diffs.last().unwrap().windows(2).map(|xs| xs[1] - xs[0]).collect();
            diffs.push(diff.clone());
            if diff.iter().all(|&x| x == 0) {
                break;
            }
        }

        diffs.last_mut().unwrap().push(0);
        
        for i in (1..diffs.len()).rev() {
            let n = diffs[i].last().unwrap() + diffs[i - 1].last().unwrap();
            diffs[i - 1].push(n); 
        }

        sum += diffs.first().unwrap().last().unwrap();
    }

    sum
}

fn part_two(input: &str) -> i64 {
    let sequences: Vec<_> = input.lines().filter_map(|line| {
        Some(line.split_whitespace().map(|s| s.parse::<i64>().ok()).collect::<Option<Vec<_>>>()?)
    }).collect();

    let mut sum = 0;

    for sequence in sequences {
        let mut diffs = vec![sequence.clone()];
        loop {
            let diff: Vec<_> = diffs.last().unwrap().windows(2).map(|xs| xs[1] - xs[0]).collect();
            diffs.push(diff.clone());
            if diff.iter().all(|&x| x == 0) {
                break;
            }
        }

        diffs.last_mut().unwrap().insert(0, 0);
        
        for i in (1..diffs.len()).rev() {
            let n = diffs[i - 1].first().unwrap() - diffs[i].first().unwrap();
            diffs[i - 1].insert(0, n); 
        }

        sum += diffs.first().unwrap().first().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_example() {
        let input = concat!(
            "0 3 6 9 12 15\n",
            "1 3 6 10 15 21\n",
            "10 13 16 21 30 45\n",
        );
        assert_eq!(part_one(input), 114);
        assert_eq!(part_two(input), 2);
    }
}
