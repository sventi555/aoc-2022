use std::ops::RangeInclusive;

use utils::read_lines;

fn ascending_range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn part_a() {
    let rock_paths: Vec<Vec<Vec<usize>>> = read_lines("./day_14/input.txt")
        .map(|line| {
            line.split(" -> ")
                .map(|coord_pair| {
                    coord_pair
                        .split(',')
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let grid_size = rock_paths
        .iter()
        .flatten()
        .fold((0, 0), |(max_x, max_y), coord_pair| {
            (
                if coord_pair[0] + 1 > max_x {
                    coord_pair[0] + 1
                } else {
                    max_x
                },
                if coord_pair[1] + 1 > max_y {
                    coord_pair[1] + 1
                } else {
                    max_y
                },
            )
        });

    let mut grid = vec![vec![0; grid_size.0]; grid_size.1];

    // set up the grid with rocks
    rock_paths.iter().for_each(|rock_path| {
        rock_path.windows(2).for_each(|line| {
            let v1 = &line[0];
            let v2 = &line[1];

            for x in ascending_range(v1[0], v2[0]) {
                for y in ascending_range(v1[1], v2[1]) {
                    grid[y][x] = 1;
                }
            }
        })
    });

    let source = (500, 0);

    let mut into_the_void = false;
    let mut time_units = 0;
    while !into_the_void {
        let mut cur_pos = source;
        loop {
            grid[cur_pos.1][cur_pos.0] = 1;
            let next_pos;
            // if we are at the bottom, then we can fall into the void
            if cur_pos.1 == grid_size.1 - 1 {
                into_the_void = true;
                break;
            } else if grid[cur_pos.1 + 1][cur_pos.0] == 0 {
                // empty space below
                next_pos = (cur_pos.0, cur_pos.1 + 1);
            } else if cur_pos.0 == 0 {
                // at the far left of the screen, fall into the void
                into_the_void = true;
                break;
            } else if grid[cur_pos.1 + 1][cur_pos.0 - 1] == 0 {
                // there is space to the bottom left
                next_pos = (cur_pos.0 - 1, cur_pos.1 + 1);
            } else if cur_pos.0 == grid_size.0 - 1 {
                // far right of the screen, fall into the void
                into_the_void = true;
                break;
            } else if grid[cur_pos.1 + 1][cur_pos.0 + 1] == 0 {
                // there is space to the bottom right
                next_pos = (cur_pos.0 + 1, cur_pos.1 + 1);
            } else {
                // no space below or on either side, landed
                time_units += 1;
                break;
            }

            grid[cur_pos.1][cur_pos.0] = 0;
            cur_pos = next_pos;
        }
    }

    println!("{}", time_units);
}

fn part_b() {
    let rock_paths: Vec<Vec<Vec<usize>>> = read_lines("./day_14/input.txt")
        .map(|line| {
            line.split(" -> ")
                .map(|coord_pair| {
                    coord_pair
                        .split(',')
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    let grid_size = rock_paths
        .iter()
        .flatten()
        .fold((0, 0), |(max_x, max_y), coord_pair| {
            (
                if coord_pair[0] + 200 > max_x {
                    coord_pair[0] + 200
                } else {
                    max_x
                },
                if coord_pair[1] + 2 > max_y {
                    coord_pair[1] + 2
                } else {
                    max_y
                },
            )
        });

    let mut grid = vec![vec![0; grid_size.0]; grid_size.1];

    // set up the grid with rocks
    rock_paths.iter().for_each(|rock_path| {
        rock_path.windows(2).for_each(|line| {
            let v1 = &line[0];
            let v2 = &line[1];

            for x in ascending_range(v1[0], v2[0]) {
                for y in ascending_range(v1[1], v2[1]) {
                    grid[y][x] = 1;
                }
            }
        })
    });

    let source = (500, 0);

    let mut blocked_source = false;
    let mut time_units = 0;
    while !blocked_source {
        let mut cur_pos = source;
        loop {
            grid[cur_pos.1][cur_pos.0] = 1;
            let next_pos;
            // if we are at the bottom, then land
            if cur_pos.1 == grid_size.1 - 1 {
                time_units += 1;
                break;
            } else if grid[cur_pos.1 + 1][cur_pos.0] == 0 {
                // empty space below
                next_pos = (cur_pos.0, cur_pos.1 + 1);
            } else if grid[cur_pos.1 + 1][cur_pos.0 - 1] == 0 {
                // there is space to the bottom left
                next_pos = (cur_pos.0 - 1, cur_pos.1 + 1);
            } else if grid[cur_pos.1 + 1][cur_pos.0 + 1] == 0 {
                // there is space to the bottom right
                next_pos = (cur_pos.0 + 1, cur_pos.1 + 1);
            } else {
                // no space below or on either side, landed
                time_units += 1;
                if cur_pos == source {
                    blocked_source = true;
                }
                break;
            }

            grid[cur_pos.1][cur_pos.0] = 0;
            cur_pos = next_pos;
        }
    }

    println!("{}", time_units);
}

fn main() {
    part_a();
    part_b();
}
