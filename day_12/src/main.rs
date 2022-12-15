use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use utils::read_lines;

struct Node {
    pos: (usize, usize),
    goal: bool,
    altitude: char,
    neighbours: Vec<Rc<RefCell<Node>>>,
}

fn bfs(start: &Rc<RefCell<Node>>) -> u32 {
    let mut queue: VecDeque<(Rc<RefCell<Node>>, u32)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    visited.insert(start.borrow().pos);
    queue.push_back((Rc::clone(start), 0));
    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        if cur.0.borrow().goal {
            return cur.1;
        }

        let cur_node = cur.0.borrow();
        let neighbours = &cur_node.neighbours;
        for neighbour in neighbours {
            let pos = neighbour.borrow().pos;
            if !visited.contains(&pos) {
                visited.insert(pos);
                queue.push_back((Rc::clone(neighbour), cur.1 + 1))
            }
        }
    }

    std::u32::MAX
}

fn part_a() {
    let mut start_index: (usize, usize) = (0, 0);

    // build nodes
    let nodes: Vec<Vec<Rc<RefCell<Node>>>> = read_lines("./day_12/input.txt")
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(col_index, c)| {
                    if c == 'S' {
                        start_index = (row_index, col_index);
                    }
                    Rc::new(RefCell::new(Node {
                        pos: (row_index, col_index),
                        goal: c == 'E',
                        altitude: if c == 'S' {
                            'a'
                        } else if c == 'E' {
                            'z'
                        } else {
                            c
                        },
                        neighbours: Vec::new(),
                    }))
                })
                .collect()
        })
        .collect();

    // build edges
    let rows = nodes.len();
    let cols = nodes[0].len();
    for row in 0..rows {
        for col in 0..cols {
            let elem = nodes[row][col].borrow().altitude as i32;

            if row > 0 {
                let above = nodes[row - 1][col].borrow().altitude as i32;
                if above - elem <= 1 {
                    nodes[row][col]
                        .borrow_mut()
                        .neighbours
                        .push(Rc::clone(&nodes[row - 1][col]));
                }
            }

            if row < rows - 1 {
                let below = nodes[row + 1][col].borrow().altitude as i32;
                if below - elem <= 1 {
                    nodes[row][col]
                        .borrow_mut()
                        .neighbours
                        .push(Rc::clone(&nodes[row + 1][col]));
                }
            }

            if col > 0 {
                let left = nodes[row][col - 1].borrow().altitude as i32;
                if left - elem <= 1 {
                    nodes[row][col]
                        .borrow_mut()
                        .neighbours
                        .push(Rc::clone(&nodes[row][col - 1]));
                }
            }

            if col < cols - 1 {
                let right = nodes[row][col + 1].borrow().altitude as i32;
                if right - elem <= 1 {
                    nodes[row][col]
                        .borrow_mut()
                        .neighbours
                        .push(Rc::clone(&nodes[row][col + 1]));
                }
            }
        }
    }

    let path_len = bfs(&nodes[start_index.0][start_index.1]);
    // let path_len = dijkstra(&nodes, start_index);
    println!("{}", path_len);
}

fn part_b() {
    let mut start_indices: Vec<(usize, usize)> = Vec::new();

    // build nodes
    let nodes: Vec<Vec<Rc<RefCell<Node>>>> = read_lines("./day_12/input.txt")
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(col_index, c)| {
                    if c == 'a' {
                        start_indices.push((row_index, col_index));
                    }
                    Rc::new(RefCell::new(Node {
                        pos: (row_index, col_index),
                        goal: c == 'E',
                        altitude: if c == 'S' {
                            'a'
                        } else if c == 'E' {
                            'z'
                        } else {
                            c
                        },
                        neighbours: Vec::new(),
                    }))
                })
                .collect()
        })
        .collect();

    // build edges
    let rows = nodes.len();
    let cols = nodes[0].len();
    for row in 0..rows {
        for col in 0..cols {
            let elem = nodes[row][col].borrow().altitude as i32;

            if row > 0 {
                let above = nodes[row - 1][col].borrow().altitude as i32;
                if above - elem <= 1 {
                    nodes[row][col]
                        .borrow_mut()
                        .neighbours
                        .push(Rc::clone(&nodes[row - 1][col]));
                }
            }

            if row < rows - 1 {
                let below = nodes[row + 1][col].borrow().altitude as i32;
                if below - elem <= 1 {
                    nodes[row][col]
                        .borrow_mut()
                        .neighbours
                        .push(Rc::clone(&nodes[row + 1][col]));
                }
            }

            if col > 0 {
                let left = nodes[row][col - 1].borrow().altitude as i32;
                if left - elem <= 1 {
                    nodes[row][col]
                        .borrow_mut()
                        .neighbours
                        .push(Rc::clone(&nodes[row][col - 1]));
                }
            }

            if col < cols - 1 {
                let right = nodes[row][col + 1].borrow().altitude as i32;
                if right - elem <= 1 {
                    nodes[row][col]
                        .borrow_mut()
                        .neighbours
                        .push(Rc::clone(&nodes[row][col + 1]));
                }
            }
        }
    }

    let shortest_path_len = start_indices
        .iter()
        .fold(std::u32::MAX, |current_min, start_index| {
            let steps = bfs(&nodes[start_index.0][start_index.1]);
            if steps < current_min {
                steps
            } else {
                current_min
            }
        });
    println!("{}", shortest_path_len);
}

fn main() {
    part_a();
    part_b();
}
