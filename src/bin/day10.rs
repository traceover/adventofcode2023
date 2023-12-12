use std::fs;
use array2d::Array2D;

pub const NORTH: u8 = 0b1000;
pub const EAST:  u8 = 0b0100;
pub const SOUTH: u8 = 0b0010;
pub const WEST:  u8 = 0b0001;

fn main() {
    let input = fs::read_to_string("inputs/10.txt").expect("Failed to read input file");
    println!("Part one: {}", part_one(&input));
}

pub fn part_one(input: &str) -> usize {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().len();

    let mut grid = Array2D::filled_with(0u8, num_rows, num_cols);
    let mut start = (0, 0);
    let mut row = 0;
    let mut col = 0;

    for ch in input.chars() {
        let tile = match ch {
            '|' => Some(0b1010), // NS
            '-' => Some(0b0101), // EW
            'L' => Some(0b1100), // NE
            'J' => Some(0b1001), // NW
            '7' => Some(0b0011), // SW
            'F' => Some(0b0110), // SE
            'S' => {
                start = (row, col);
                None
            },
            '\n' => {
                row += 1;
                col = 0;
                continue;
            },
            _ => None,
        };

        if let Some(tile) = tile {
            if grid.set(row, col, tile).is_err() {
                eprintln!("ERROR: indices ({row}, {col}) out of bounds");
            }
        }
        col += 1;
    }

    // Calculate the tile under the starting position
    let mut nbors = 0u8;
    if start.0 > 0 && grid[(start.0 - 1, start.1)] & SOUTH > 0 {
        nbors |= NORTH;
    }
    if grid[(start.0 + 1, start.1)] & NORTH > 0 {
        nbors |= SOUTH;
    }
    if grid[(start.0, start.1 + 1)] & WEST > 0 {
        nbors |= EAST;
    }
    if start.1 > 0 && grid[(start.0, start.1 - 1)] & EAST > 0 {
        nbors |= WEST;
    }
    let _ = grid.set(start.0, start.1, nbors);

    // Second try...
    let mut visited = Array2D::filled_with(false, num_rows, num_cols);
    let mut points = vec![start];
    let mut steps = 0;

    loop {
        let mut next_points = vec![];
        for (row, col) in points {
            visited[(row, col)] = true;
            let nbors = grid[(row, col)];
            if nbors & NORTH > 0 && row > 0 && !visited[(row - 1, col)] {
                next_points.push((row - 1, col));
            }
            if nbors & SOUTH > 0 && !visited[(row + 1, col)] {
                next_points.push((row + 1, col));
            }
            if nbors & WEST > 0 && col > 0 && !visited[(row, col - 1)] {
                next_points.push((row, col - 1));
            }
            if nbors & EAST > 0 && !visited[(row, col + 1)] {
                next_points.push((row, col + 1));
            }
        }
        if next_points.len() == 0 {
            break;
        }
        points = next_points;
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::{part_one};

    #[test]
    fn test_example_1() {
        let input = concat!(
            ".....\n",
            ".S-7.\n",
            ".|.|.\n",
            ".L-J.\n",
            ".....\n",
        );
        assert_eq!(part_one(input), 4);
    }

    #[test]
    fn test_example_2() {
        let input = concat!(
            "7-F7-\n",
            ".FJ|7\n",
            "SJLL7\n",
            "|F--J\n",
            "LJ.LJ\n",
        );
        assert_eq!(part_one(input), 8);
    }
}
