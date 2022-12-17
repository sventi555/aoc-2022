use std::collections::{HashMap, HashSet, VecDeque};

use utils::read_lines;

struct Node {
    index: usize,
    name: String,
    rate: u32,
    neighbours: Vec<String>,
}

// ok so i had to copy a lot of this from someone, but i was DEFINITELY on the right
// track with my first instincts for dynamic programming. I just didn't make the
// initial effort to prune the search space effectively, and wouldn't have been
// smart enough to use a queue (or a hash map for that matter...). This was
// good practice!
fn part_a() {
    let nodes: Vec<Node> = read_lines("./day_16/input.txt")
        .enumerate()
        .map(|(index, line)| {
            let parts: Vec<&str> = line.split(';').collect();

            let left: Vec<&str> = parts[0].split(' ').collect();
            let name = left[1].to_string();
            let rate: u32 = left
                .last()
                .unwrap()
                .split('=')
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let right: Vec<&str> = parts[1].split(' ').collect();
            let neighbours: Vec<String> = right[5..]
                .iter()
                .map(|name| {
                    if name.ends_with(',') {
                        String::from(name.strip_suffix(',').unwrap())
                    } else {
                        String::from(*name)
                    }
                })
                .collect();

            Node {
                index,
                name,
                rate,
                neighbours,
            }
        })
        .collect();

    let mut node_map: HashMap<String, &Node> = HashMap::new();
    nodes.iter().for_each(|n| {
        node_map.insert(n.name.clone(), &n);
    });

    // build adjacency graph
    let mut distances = vec![vec![10000; nodes.len()]; nodes.len()];
    for node in &nodes {
        let node_index = node.index;
        let neighbours = &node.neighbours;

        distances[node_index][node_index] = 0;
        for neighbour in neighbours {
            let neighbour_index = node_map.get(neighbour).unwrap().index;
            distances[node_index][neighbour_index] = 1;
        }
    }

    // turn adjacency graph into distance graph (floyd warshal)
    let n = distances.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if distances[i][k] + distances[k][j] < distances[i][j] {
                    distances[i][j] = distances[i][k] + distances[k][j];
                }
            }
        }
    }

    let positive_rate_nodes: Vec<usize> = nodes
        .iter()
        .filter(|n| n.rate > 0)
        .map(|n| n.index)
        .collect();

    // node -> time -> nodes turned on (repr as sum of 2^node_index)
    let mut dp: HashMap<(usize, u32, u64), u32> = HashMap::new();
    let mut queue: VecDeque<(usize, u32, u64)> = VecDeque::new();

    let start_node = node_map.get(&"AA".to_string()).unwrap();
    let dp_index = (start_node.index, 30, 0);
    queue.push_back(dp_index);
    dp.insert(dp_index, 0);

    while !queue.is_empty() {
        let (node_index, time, turned_on) = queue.pop_front().unwrap();
        let total_flow = *dp.get(&(node_index, time, turned_on)).unwrap();

        let mut add_to_queue = |dp_index: (usize, u32, u64), val: u32| {
            if !dp.contains_key(&dp_index) || *dp.get(&dp_index).unwrap() < val {
                queue.push_back(dp_index);
                dp.insert(dp_index, val);
            }
        };

        // if not turned on and there is time to turn it on, add that case to queue
        if turned_on & (1 << node_index) == 0 && time >= 1 {
            let node_flow = (time - 1) * nodes[node_index].rate;
            let dp_index = (node_index, time - 1, turned_on | (1 << node_index));
            add_to_queue(dp_index, node_flow + total_flow);
        }

        for next_node_index in positive_rate_nodes.iter().copied() {
            if next_node_index != node_index {
                let dist = distances[node_index][next_node_index];
                if dist <= time {
                    let dp_index = (next_node_index, time - dist, turned_on);
                    add_to_queue(dp_index, total_flow);
                }
            }
        }
    }

    let max_flow = dp.values().max().unwrap();

    println!("{}", max_flow);
}

fn part_b() {
    let nodes: Vec<Node> = read_lines("./day_16/input.txt")
        .enumerate()
        .map(|(index, line)| {
            let parts: Vec<&str> = line.split(';').collect();

            let left: Vec<&str> = parts[0].split(' ').collect();
            let name = left[1].to_string();
            let rate: u32 = left
                .last()
                .unwrap()
                .split('=')
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let right: Vec<&str> = parts[1].split(' ').collect();
            let neighbours: Vec<String> = right[5..]
                .iter()
                .map(|name| {
                    if name.ends_with(',') {
                        String::from(name.strip_suffix(',').unwrap())
                    } else {
                        String::from(*name)
                    }
                })
                .collect();

            Node {
                index,
                name,
                rate,
                neighbours,
            }
        })
        .collect();

    let mut node_map: HashMap<String, &Node> = HashMap::new();
    nodes.iter().for_each(|n| {
        node_map.insert(n.name.clone(), &n);
    });

    // build adjacency graph
    let mut distances = vec![vec![10000; nodes.len()]; nodes.len()];
    for node in &nodes {
        let node_index = node.index;
        let neighbours = &node.neighbours;

        distances[node_index][node_index] = 0;
        for neighbour in neighbours {
            let neighbour_index = node_map.get(neighbour).unwrap().index;
            distances[node_index][neighbour_index] = 1;
        }
    }

    // turn adjacency graph into distance graph (floyd warshal)
    let n = distances.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if distances[i][k] + distances[k][j] < distances[i][j] {
                    distances[i][j] = distances[i][k] + distances[k][j];
                }
            }
        }
    }

    let positive_rate_nodes: Vec<usize> = nodes
        .iter()
        .filter(|n| n.name == "AA" || n.rate > 0)
        .map(|n| n.index)
        .collect();

    let index_in_pos_nodes = |original_index| {
        positive_rate_nodes
            .iter()
            .enumerate()
            .find(|(_, node_index)| **node_index == original_index)
            .unwrap()
            .0
    };

    // node -> time -> nodes turned on (repr as sum of 2^node_index)
    let mut dp: HashMap<(usize, u32, u64), u32> = HashMap::new();
    let mut queue: VecDeque<(usize, u32, u64)> = VecDeque::new();

    let start_node = node_map.get(&"AA".to_string()).unwrap();
    let dp_index = (start_node.index, 26, 0);
    queue.push_back(dp_index);
    dp.insert(dp_index, 0);

    while !queue.is_empty() {
        let (node_index, time, turned_on) = queue.pop_front().unwrap();
        let total_flow = *dp.get(&(node_index, time, turned_on)).unwrap();

        let mut add_to_queue = |dp_index: (usize, u32, u64), val: u32| {
            if !dp.contains_key(&dp_index) || *dp.get(&dp_index).unwrap() < val {
                queue.push_back(dp_index);
                dp.insert(dp_index, val);
            }
        };

        // if not turned on and there is time to turn it on, add that case to queue
        if turned_on & (1 << index_in_pos_nodes(node_index)) == 0 && time >= 1 {
            let node_flow = (time - 1) * nodes[node_index].rate;
            let dp_index = (
                node_index,
                time - 1,
                turned_on | (1 << index_in_pos_nodes(node_index)),
            );
            add_to_queue(dp_index, node_flow + total_flow);
        }

        for next_node_index in positive_rate_nodes.iter().copied() {
            if next_node_index != node_index {
                let dist = distances[node_index][next_node_index];
                if dist <= time {
                    let dp_index = (next_node_index, time - dist, turned_on);
                    add_to_queue(dp_index, total_flow);
                }
            }
        }
    }

    let m = positive_rate_nodes.len();
    let mut table = vec![0; 1 << m];
    for (dp_index, val) in dp.iter() {
        table[dp_index.2 as usize] = table[dp_index.2 as usize].max(*val);
    }

    let mut ret = 0;
    for mask in 0..(1 << m) {
        let mask3 = ((1 << m) - 1) ^ mask;
        ret = ret.max(table[mask3]);
        let mut mask2 = mask;
        while mask2 > 0 {
            println!("{:b}, {}", mask2, table[mask2]);
            ret = ret.max(table[mask3] + table[mask2]);
            mask2 = (mask2 - 1) & mask
        }
    }

    println!("{ret}");
}

fn main() {
    part_a();
    part_b();
}
