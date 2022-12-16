use std::collections::HashSet;

use utils::{abs, read_lines};

struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    fn dist(&self, other: &Pos) -> i32 {
        abs(self.x - other.x) + abs(self.y - other.y)
    }
}

struct Diamond {
    sensor: Pos,
    beacon: Pos,
}

impl Diamond {
    fn radius(&self) -> i32 {
        self.sensor.dist(&self.beacon)
    }

    fn contains(&self, p: &Pos) -> bool {
        self.sensor.dist(p) <= self.radius()
    }
}

impl From<String> for Diamond {
    fn from(s: String) -> Self {
        let coords: Vec<Vec<i32>> = s
            .split(':')
            .map(|coord| {
                coord
                    .split(',')
                    .map(|val| {
                        val.split('=')
                            .collect::<Vec<&str>>()
                            .last()
                            .unwrap()
                            .parse()
                            .unwrap()
                    })
                    .collect()
            })
            .collect();

        Diamond {
            sensor: Pos::new(coords[0][0], coords[0][1]),
            beacon: Pos::new(coords[1][0], coords[1][1]),
        }
    }
}

fn part_a() {
    let diamonds: Vec<Diamond> = read_lines("./day_15/input.txt")
        .map(|line| line.into())
        .collect();

    let mut not_beacon: HashSet<i32> = HashSet::new();

    let target_row = 2000000;
    for diamond in diamonds {
        let cart_dist = diamond.radius();
        let dist_to_target_row = abs(diamond.sensor.y - target_row);
        if dist_to_target_row <= cart_dist {
            let width = cart_dist - dist_to_target_row;
            // slight optimization
            if not_beacon.contains(&(diamond.sensor.x + width))
                && not_beacon.contains(&(diamond.sensor.x - width))
            {
                continue;
            }

            for i in 0..=(cart_dist - dist_to_target_row) {
                not_beacon.insert(diamond.sensor.x + i);
                not_beacon.insert(diamond.sensor.x - i);
            }
        }
        if diamond.beacon.y == target_row {
            not_beacon.remove(&diamond.beacon.x);
        }
    }

    println!("{}", not_beacon.len());
}

fn part_b() {
    let diamonds: Vec<Diamond> = read_lines("./day_15/input.txt")
        .map(|line| line.into())
        .collect();

    // For each diamond, start at one corner and work around the exterior perimeter.
    // At each iteration, check if we are touching another diamond,
    // and skip ahead to the end of whichever one ends first in the current direction.
    // If we ever get to a point where we don't find an adjacent (or surrounding) square,
    // then that's our answer!

    // HAHA nevermind, don't even need to do the skipping ahead bits. Rust is just
    // stinky fast, so we're chillin for this input size (and many magnitudes greater)

    // also this code could definitely be cleaned up but I ain't doin it rn
    let mut tuning_freq: i64 = 0;

    'outer: for diamond in &diamonds {
        let mut cur_pos = Pos::new(diamond.sensor.x, diamond.sensor.y - diamond.radius() - 1);

        // top right edge (x++, y++)
        while cur_pos.y < diamond.sensor.y {
            if cur_pos.x < 0 || cur_pos.x > 4000000 || cur_pos.y < 0 || cur_pos.y > 4000000 {
            } else {
                let adjacent = diamonds.iter().find(|diamond| diamond.contains(&cur_pos));
                if adjacent.is_none() {
                    tuning_freq = cur_pos.x as i64 * 4000000 + cur_pos.y as i64;
                    break 'outer;
                }
            }

            cur_pos.x += 1;
            cur_pos.y += 1;
        }

        // bottom right edge (x--, y++)
        while cur_pos.x > diamond.sensor.x {
            if cur_pos.x < 0 || cur_pos.x > 4000000 || cur_pos.y < 0 || cur_pos.y > 4000000 {
            } else {
                let adjacent = diamonds.iter().find(|diamond| diamond.contains(&cur_pos));
                if adjacent.is_none() {
                    tuning_freq = cur_pos.x as i64 * 4000000 + cur_pos.y as i64;
                    break 'outer;
                }
            }

            cur_pos.x -= 1;
            cur_pos.y += 1;
        }

        // bottom left edge (x--, y--)
        while cur_pos.y > diamond.sensor.y {
            if cur_pos.x < 0 || cur_pos.x > 4000000 || cur_pos.y < 0 || cur_pos.y > 4000000 {
            } else {
                let adjacent = diamonds.iter().find(|diamond| diamond.contains(&cur_pos));
                if adjacent.is_none() {
                    tuning_freq = cur_pos.x as i64 * 4000000 + cur_pos.y as i64;
                    break 'outer;
                }
            }

            cur_pos.x -= 1;
            cur_pos.y -= 1;
        }

        // top left edge (x++, y--)
        while cur_pos.x < diamond.sensor.x {
            if cur_pos.x < 0 || cur_pos.x > 4000000 || cur_pos.y < 0 || cur_pos.y > 4000000 {
            } else {
                let adjacent = diamonds.iter().find(|diamond| diamond.contains(&cur_pos));
                if adjacent.is_none() {
                    tuning_freq = cur_pos.x as i64 * 4000000 + cur_pos.y as i64;
                    break 'outer;
                }
            }

            cur_pos.x += 1;
            cur_pos.y -= 1;
        }
    }

    println!("{}", tuning_freq);
}

fn main() {
    part_a();
    part_b();
}
