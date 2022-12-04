use std::collections::{HashMap, HashSet};
use utils::read_lines;

fn ascii_to_prio(c: &char) -> Option<u32> {
    let code = *c as u32;

    match code {
        65..=90 => Some(code - 38),
        97..=122 => Some(code - 96),
        _ => None,
    }
}

fn part_a() {
    let prio_sum: u32 = read_lines("./day_3/input.txt")
        .map(|bag| {
            let num_items = bag.len();

            let mut first_half = HashSet::new();
            for (i, c) in bag.chars().enumerate() {
                if i < num_items / 2 {
                    first_half.insert(c);
                } else if first_half.contains(&c) {
                    return ascii_to_prio(&c).unwrap();
                }
            }

            panic!();
        })
        .sum();

    println!("{}", prio_sum);
}

fn part_b() {
    let mut lines = read_lines("./day_3/input.txt");

    let mut badge_sum = 0;
    let mut done = false;
    while !done {
        let mut item_counts: HashMap<char, u32> = HashMap::new();
        for _ in 0..3 {
            let mut bag_contents: HashSet<char> = HashSet::new();
            let line = lines.next();
            if let Some(bag) = line {
                for c in bag.chars() {
                    if !bag_contents.contains(&c) {
                        item_counts
                            .entry(c)
                            .and_modify(|item_count| *item_count += 1)
                            .or_insert(1);
                    }
                    bag_contents.insert(c);
                }
            } else {
                done = true;
                break;
            }
        }

        if !done {
            let badge_char = item_counts
                .into_iter()
                .find(|entry| entry.1 == 3)
                .unwrap()
                .0;
            badge_sum += ascii_to_prio(&badge_char).unwrap();
        }
    }

    println!("{}", badge_sum);
}

fn part_b_alt() {
    // INSPIRATION
    // let mut sets: Vec<HashSet<char>> = Vec::new();
    // sets.push(['a', 'b', 'c', 'd'].iter().cloned().collect());
    // sets.push(['c', 'd'].iter().cloned().collect());
    // sets.push(['d', 'a'].iter().cloned().collect());

    // let intersection = sets.iter().skip(1).fold(sets[0].clone(), |acc, hs| {
    //     acc.intersection(hs).cloned().collect()
    // });

    let badge_sum = read_lines("./day_3/input.txt")
        .fold(Vec::new(), |mut groups: Vec<Vec<String>>, member| {
            if groups.is_empty() || groups.last().unwrap().len() == 3 {
                groups.push(Vec::new());
            }

            groups.last_mut().unwrap().push(member);
            groups
        })
        .iter()
        .fold(0, |count, group| {
            let bag_1: HashSet<char> = HashSet::from_iter(group[0].chars());
            let bag_2: HashSet<char> = HashSet::from_iter(group[1].chars());
            let bag_3: HashSet<char> = HashSet::from_iter(group[2].chars());

            let inter_1: HashSet<char> = bag_1.intersection(&bag_2).cloned().collect();
            let inter_2: Vec<char> = inter_1.intersection(&bag_3).cloned().collect();

            count + ascii_to_prio(&inter_2[0]).unwrap()
        });

    println!("{}", badge_sum);
}

fn main() {
    part_a();
    // part_b();
    part_b_alt();
}
