use std::cmp::max;

use utils::read_lines;

fn part_a() {
    let grid: Vec<Vec<u32>> = read_lines("./day_8/input.txt")
        .map(|line| {
            line.chars()
                .map(|c| String::from(c).parse().unwrap())
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut visible_grid: Vec<Vec<u32>> = Vec::new();
    for _ in 0..rows {
        visible_grid.push(Vec::new());
        for _ in 0..cols {
            visible_grid.last_mut().unwrap().push(0);
        }
    }

    // viewing from left and right
    for row in 0..rows {
        let mut cur_tallest = grid[row][0];
        visible_grid[row][0] = 1;
        for col in 0..cols {
            if grid[row][col] > cur_tallest {
                cur_tallest = grid[row][col];
                visible_grid[row][col] = 1;
            }
        }

        cur_tallest = *grid[row].last().unwrap();
        *visible_grid[row].last_mut().unwrap() = 1;
        for col in (0..cols).rev() {
            if grid[row][col] > cur_tallest {
                cur_tallest = grid[row][col];
                visible_grid[row][col] = 1;
            }
        }
    }

    // viewing from top and bottom
    for col in 0..cols {
        let mut cur_tallest = grid[0][col];
        visible_grid[0][col] = 1;
        for row in 0..rows {
            if grid[row][col] > cur_tallest {
                cur_tallest = grid[row][col];
                visible_grid[row][col] = 1;
            }
        }

        cur_tallest = grid.last().unwrap()[col];
        visible_grid.last_mut().unwrap()[col] = 1;
        for row in (0..rows).rev() {
            if grid[row][col] > cur_tallest {
                cur_tallest = grid[row][col];
                visible_grid[row][col] = 1;
            }
        }
    }

    let total_visible: u32 = visible_grid.iter().flatten().sum();

    println!("{}", total_visible);
}

fn part_b() {
    let grid: Vec<Vec<u32>> = read_lines("./day_8/input.txt")
        .map(|line| {
            line.chars()
                .map(|c| String::from(c).parse().unwrap())
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut max_scenic_score = 0;
    for row in 0..rows {
        for col in 0..cols {
            if row == 0 || row == rows - 1 || col == 0 || col == cols - 1 {
                continue;
            }

            let elem = grid[row][col];

            // while there is something adjacent
            // if the adjacent thing is <= elem, add one to tracker
            // if thing is >= elem, break
            let mut left_tracker = 0;
            while left_tracker < col {
                let c = grid[row][col - left_tracker - 1].cmp(&elem);
                if c.is_le() {
                    left_tracker += 1
                }
                if c.is_ge() {
                    break;
                }
            }

            let mut right_tracker = 0;
            while right_tracker < (cols - col) - 1 {
                let c = grid[row][col + right_tracker + 1].cmp(&elem);
                if c.is_le() {
                    right_tracker += 1
                }
                if c.is_ge() {
                    break;
                }
            }

            let mut up_tracker = 0;
            while up_tracker < row {
                let c = grid[row - up_tracker - 1][col].cmp(&elem);
                if c.is_le() {
                    up_tracker += 1
                }
                if c.is_ge() {
                    break;
                }
            }

            let mut down_tracker = 0;
            while down_tracker < (rows - row) - 1 {
                let c = grid[row + down_tracker + 1][col].cmp(&elem);
                if c.is_le() {
                    down_tracker += 1
                }
                if c.is_ge() {
                    break;
                }
            }

            let scenic_score = left_tracker * right_tracker * up_tracker * down_tracker;
            max_scenic_score = max(scenic_score, max_scenic_score);
        }
    }

    println!("{}", max_scenic_score);
}

fn main() {
    part_a();
    part_b();
}
