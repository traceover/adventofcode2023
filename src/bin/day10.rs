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

    /*
    // Find the maximum distance by following the neighbors
    let mut max_dist = 0;
    let mut distances = Array2D::filled_with(0, num_rows, num_cols);
    dfs(&grid, &mut distances, start.0, start.1, start.0, start.1, 0.0, 0, &mut max_dist);
    let _ = distances[(start.0, start.1)] = 99;
    
    for row in 0..num_rows {
        for col in 0..num_cols {
            print!("{:2}", distances[(row, col)]);
        }
        print!("\n");
    }

    max_dist
    */
}

pub fn dfs(
    grid: &Array2D<u8>,
    distances: &mut Array2D<usize>,
    start_row: usize, start_col: usize,
    row: usize, col: usize,
    dist: f32,
    steps: usize, max_steps: &mut usize
) {
    *max_steps = std::cmp::max(*max_steps, steps);

    let _ = distances.set(row, col, steps);

    let nbors = grid[(row, col)];

    if nbors & NORTH > 0 && row > 0 {
        let row = row - 1;
        let new_dist = distance(start_row, start_col, row, col);
        if new_dist > dist {
            dfs(grid, distances, start_row, start_col, row, col, new_dist, steps + 1, max_steps);
        }
    }
    if nbors & EAST > 0 {
        let col = col + 1;
        let new_dist = distance(start_row, start_col, row, col);
        if new_dist > dist {
            dfs(grid, distances, start_row, start_col, row, col, new_dist, steps + 1, max_steps);
        }
    }
    if nbors & SOUTH > 0 {
        let row = row + 1;
        let new_dist = distance(start_row, start_col, row, col);
        if new_dist > dist {
            dfs(grid, distances, start_row, start_col, row, col, new_dist, steps + 1, max_steps);
        }
    }
    if nbors & WEST > 0 && col > 0 {
        let col = col - 1;
        let new_dist = distance(start_row, start_col, row, col);
        if new_dist > dist {
            dfs(grid, distances, start_row, start_col, row, col, new_dist, steps + 1, max_steps);
        }
    }
}

fn distance(x0: usize, y0: usize, x1: usize, y1: usize) -> f32 {
    let dx = x1 as f32 - x0 as f32;
    let dy = y1 as f32 - y0 as f32;
    (dx*dx + dy*dy).sqrt()
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
