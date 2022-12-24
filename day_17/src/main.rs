use core::panic;
use std::collections::{HashMap, HashSet};

use utils::read_lines;

enum Dir {
    Left,
    Right,
    Down,
}

struct JetStream {
    pattern: Vec<Dir>,
    cur: usize,
}

impl From<String> for JetStream {
    fn from(s: String) -> Self {
        let pattern = s
            .chars()
            .map(|c| match c {
                '<' => Dir::Left,
                '>' => Dir::Right,
                _ => panic!(),
            })
            .collect();
        JetStream { pattern, cur: 0 }
    }
}

impl JetStream {
    fn next_dir(&mut self) -> &Dir {
        let dir = &self.pattern[self.cur];
        self.cur = (self.cur + 1) % self.pattern.len();
        dir
    }
}

fn shift(block: &Vec<(i32, i32)>, dir: &Dir) -> Vec<(i32, i32)> {
    block
        .iter()
        .map(|coord| match dir {
            Dir::Left => (coord.0 - 1, coord.1),
            Dir::Right => (coord.0 + 1, coord.1),
            Dir::Down => (coord.0, coord.1 - 1),
        })
        .collect()
}

fn set_x(block: &Vec<(i32, i32)>, val: i32) -> Vec<(i32, i32)> {
    block.iter().map(|coord| (coord.0 + val, coord.1)).collect()
}

fn set_y(block: &Vec<(i32, i32)>, val: i32) -> Vec<(i32, i32)> {
    block.iter().map(|coord| (coord.0, coord.1 + val)).collect()
}

fn top(block: &Vec<(i32, i32)>) -> i32 {
    block.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1
}

fn simulate(t: usize) -> usize {
    let blocks = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
    let mut cur_block = 0;

    let mut jet_stream: JetStream = read_lines("./day_17/input.txt").next().unwrap().into();

    let mut shaft = vec![vec![false; 7]; t * 4];
    let shaft_len = shaft.len();

    let mut states: HashMap<(usize, usize, Vec<Vec<bool>>), usize> = HashMap::new();

    let mut max_height = 1;
    for i in 0..t {
        let mut block = set_x(&set_y(&blocks[cur_block], max_height + 3), 2);

        // find where the cycle begins and the length of the cycle (i.e. 15, 1050)
        // Get the height of the tower at 14, then add the height of the cycle
        // n = (10000000 - 15) / cycle length times. Finally, add the height of
        // the tower that's (10000000 - 15) % cycle blocks

        let cur_state = (
            cur_block,
            jet_stream.cur,
            shaft[shaft_len - max_height as usize
                ..shaft_len - max_height as usize + 100.min(max_height as usize)]
                .iter()
                .cloned()
                .collect(),
        );
        if states.contains_key(&cur_state) {
            println!(
                "i: {}, prev: {}, diff: {}",
                i,
                states.get(&cur_state).unwrap(),
                i - states.get(&cur_state).unwrap()
            );
        } else {
            states.insert(cur_state, i);
        }

        loop {
            let jet_dir = jet_stream.next_dir();

            // shift the block horizontally if possible
            let shifted = shift(&block, jet_dir);
            let mut able_to_move = true;
            for coord in &shifted {
                if coord.0 < 0
                    || coord.0 > 6
                    || shaft[shaft_len - 1 - coord.1 as usize][coord.0 as usize]
                {
                    able_to_move = false;
                    break;
                }
            }

            if able_to_move {
                block = shifted;
            }

            // shift the block down if possible
            let shifted = shift(&block, &Dir::Down);
            let mut able_to_move = true;
            for coord in &shifted {
                if coord.1 < 0 || shaft[shaft_len - 1 - coord.1 as usize][coord.0 as usize] {
                    able_to_move = false;
                    break;
                }
            }

            if able_to_move {
                block = shifted;
            } else {
                break;
            }
        }

        for coord in &block {
            shaft[shaft_len - 1 - coord.1 as usize][coord.0 as usize] = true;
        }

        max_height = max_height.max(top(&block) + 1);
        cur_block = (cur_block + 1) % blocks.len();
    }

    // for (index, line) in shaft.iter().enumerate() {
    //     print!("{} ", shaft_len - index - 1);
    //     for item in line {
    //         if *item {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    max_height as usize
}

fn part_a() {
    let max_height = simulate(2022);
    println!("{max_height}");
}

fn part_b() {
    // too lazy to code this, just found these numbers manually
    // let cycle_start = 215;
    // let max_height_start = simulate(cycle_start);

    // let cycle_len = 1725;
    // let max_height_cycle = simulate(cycle_len + cycle_start) - max_height_start;

    // let times = (1000000000000 - cycle_start) / cycle_len;
    // let leftover = (1000000000000 - cycle_start) % cycle_len;

    // let max_height_leftover = simulate(leftover);

    // let total_max_height = max_height_start + max_height_cycle * times + max_height_leftover;

    simulate(10000);

    // println!("{total_max_height}");
}

fn main() {
    part_a();
    // part_b();
}
