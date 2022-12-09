use std::{cell::RefCell, collections::HashSet, str::FromStr};

use utils::read_lines;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Segment {
    x: i32,
    y: i32,
}

impl Segment {
    fn follow(&mut self, head: &Segment) {
        let x_diff = head.x - self.x;
        let y_diff = head.y - self.y;
        if abs(x_diff) > 1 || abs(y_diff) > 1 {
            let x_steps = if abs(x_diff) == 2 { x_diff / 2 } else { x_diff };
            let y_steps = if abs(y_diff) == 2 { y_diff / 2 } else { y_diff };

            self.x += x_steps;
            self.y += y_steps;
        }
    }
}

struct Rope {
    segments: Vec<RefCell<Segment>>,
}

impl Rope {
    fn move_head(&self, instr: Instr, visited: &mut HashSet<Segment>) {
        let head = &self.segments[0];
        for _ in 0..instr.steps {
            match instr.dir {
                Dir::U => {
                    head.borrow_mut().y -= 1;
                }
                Dir::R => {
                    head.borrow_mut().x += 1;
                }
                Dir::D => {
                    head.borrow_mut().y += 1;
                }
                Dir::L => {
                    head.borrow_mut().x -= 1;
                }
            }

            for i in 1..self.segments.len() {
                let head = self.segments[i - 1].borrow();
                let mut tail = self.segments[i].borrow_mut();
                tail.follow(&head);
            }
            visited.insert(self.segments.last().unwrap().borrow().clone());
        }
    }
}

enum Dir {
    U,
    R,
    D,
    L,
}

struct Instr {
    dir: Dir,
    steps: u32,
}

impl FromStr for Instr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let dir = match parts[0] {
            "U" => Dir::U,
            "R" => Dir::R,
            "D" => Dir::D,
            "L" => Dir::L,
            _ => {
                return Err(String::from("unable to parse instruction"));
            }
        };
        Ok(Instr {
            dir,
            steps: parts[1].parse().unwrap(),
        })
    }
}

fn abs(num: i32) -> i32 {
    if num < 0 {
        -num
    } else {
        num
    }
}

fn part_a() {
    let mut segments = Vec::new();
    for _ in 0..2 {
        segments.push(RefCell::new(Segment { x: 0, y: 0 }));
    }
    let rope = Rope { segments };

    let mut visited: HashSet<Segment> = HashSet::new();
    // visited.insert(rope.clone());

    read_lines("./day_9/input.txt")
        .map(|line| Instr::from_str(&line).unwrap())
        .for_each(|instr| {
            rope.move_head(instr, &mut visited);
        });

    let num_visited = visited.len();
    println!("{}", num_visited);
}

fn part_b() {
    let mut segments = Vec::new();
    for _ in 0..10 {
        segments.push(RefCell::new(Segment { x: 0, y: 0 }));
    }
    let rope = Rope { segments };

    let mut visited: HashSet<Segment> = HashSet::new();
    // visited.insert(rope.clone());

    read_lines("./day_9/input.txt")
        .map(|line| Instr::from_str(&line).unwrap())
        .for_each(|instr| {
            rope.move_head(instr, &mut visited);
        });

    let num_visited = visited.len();
    println!("{}", num_visited);
}

fn main() {
    part_a();
    part_b();
}
