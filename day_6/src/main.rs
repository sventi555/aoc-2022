use std::collections::{HashSet, VecDeque};

use utils::read_lines;

fn are_elements_distinct(v: &VecDeque<char>) -> bool {
    let mut elems = HashSet::new();

    for elem in v {
        if elems.contains(&elem) {
            return false;
        }
        elems.insert(elem);
    }

    true
}

fn part_a() {
    let mut cur_marker: VecDeque<char> = VecDeque::new();

    let lines: Vec<String> = read_lines("./day_6/input.txt").collect();
    let mut start_index = 0;
    for (index, c) in lines[0].chars().enumerate() {
        if cur_marker.len() < 4 {
            cur_marker.push_back(c)
        } else if are_elements_distinct(&cur_marker) {
            start_index = index;
            break;
        } else {
            cur_marker.pop_front();
            cur_marker.push_back(c);
        }
    }

    println!("{}", start_index);
}

fn part_b() {
    let mut cur_marker: VecDeque<char> = VecDeque::new();

    let lines: Vec<String> = read_lines("./day_6/input.txt").collect();
    let mut start_index = 0;
    for (index, c) in lines[0].chars().enumerate() {
        if cur_marker.len() < 14 {
            cur_marker.push_back(c)
        } else if are_elements_distinct(&cur_marker) {
            start_index = index;
            break;
        } else {
            cur_marker.pop_front();
            cur_marker.push_back(c);
        }
    }

    println!("{}", start_index);
}

fn main() {
    part_a();
    part_b();
}
