use std::collections::{HashSet, VecDeque};

use utils::read_lines;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Pos { x, y, z }
    }

    fn adjacent(&self) -> Vec<Self> {
        vec![
            Pos::new(self.x - 1, self.y, self.z),
            Pos::new(self.x + 1, self.y, self.z),
            Pos::new(self.x, self.y - 1, self.z),
            Pos::new(self.x, self.y + 1, self.z),
            Pos::new(self.x, self.y, self.z - 1),
            Pos::new(self.x, self.y, self.z + 1),
        ]
    }
}

impl From<String> for Pos {
    fn from(s: String) -> Self {
        let parts: Vec<i32> = s.split(',').map(|num| num.parse().unwrap()).collect();
        Pos {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        }
    }
}

fn part_a() {
    let cubes: HashSet<Pos> = read_lines("./day_18/input.txt")
        .map(|line| line.into())
        .collect();

    let mut surface_area = 0;
    for cube in &cubes {
        for adj in cube.adjacent() {
            if !cubes.contains(&adj) {
                surface_area += 1;
            }
        }
    }

    println!("{surface_area}");
}

// returns true if start is an exterior edge
fn bfs(cubes: &HashSet<Pos>, start: &Pos) -> bool {
    let mut queue: VecDeque<Pos> = VecDeque::new();
    queue.push_back(start.clone());

    let mut visited: HashSet<Pos> = HashSet::new();

    let goal = Pos::new(20, 20, 20);
    while !queue.is_empty() {
        let cur = queue.pop_front().unwrap();
        // println!("{cur:#?}");
        if cur == goal {
            return true;
        }

        // if it's an open space, then add it to the queue
        if !visited.contains(&cur) {
            for adj in cur.adjacent() {
                if !cubes.contains(&adj) {
                    queue.push_back(adj);
                }
            }
            visited.insert(cur.clone());
        }
    }

    return false;
}

fn part_b() {
    let cubes: HashSet<Pos> = read_lines("./day_18/input.txt")
        .map(|line| line.into())
        .collect();

    let mut surface_area = 0;
    for cube in &cubes {
        for adj in &cube.adjacent() {
            if !cubes.contains(adj) {
                // bfs to 20,20,20. If you arrive, it's exterior, so add to
                // surface area.
                // This is pretty dang slow and has a lot of duplicate work,
                // but it's correct, so :shrug:
                if bfs(&cubes, adj) {
                    surface_area += 1;
                }
            }
        }
    }

    println!("{surface_area}");
}

fn main() {
    part_a();
    part_b();
}
