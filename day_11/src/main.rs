use std::{cell::RefCell, num::ParseIntError, str::FromStr};

use utils::read_lines;

enum OpVal {
    NUM(u32),
    OLD,
}

impl FromStr for OpVal {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(OpVal::OLD),
            _ => Ok(OpVal::NUM(s.parse().unwrap())),
        }
    }
}

enum OpFn {
    ADD,
    MULT,
}

impl FromStr for OpFn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(OpFn::ADD),
            "*" => Ok(OpFn::MULT),
            _ => Err(String::from("failed to parse OpFn")),
        }
    }
}

struct Monkey {
    items: Vec<u64>,
    op_fn: OpFn,
    op_val: OpVal,
    test_val: u32,
    throw_to: (u32, u32),
    inspected: u32,
}

impl Monkey {
    fn take_turn_a(&mut self, monkeys: &Vec<RefCell<Monkey>>) {
        self.worry();
        self.chill();
        self.inspected += self.items.len() as u32;
        self.throw_a(monkeys);
    }

    fn take_turn_b(&mut self, monkeys: &Vec<RefCell<Monkey>>, modulo: u64) {
        // don't chill out, and use modulo arithmetic to prevent overflow
        self.worry();
        self.inspected += self.items.len() as u32;
        self.throw_b(monkeys, modulo);
    }

    fn worry(&mut self) {
        self.items = self
            .items
            .iter()
            .map(|item| {
                let op_val = match self.op_val {
                    OpVal::OLD => *item,
                    OpVal::NUM(val) => val as u64,
                };
                match self.op_fn {
                    OpFn::ADD => item + op_val,
                    OpFn::MULT => item * op_val,
                }
            })
            .collect();
    }

    fn chill(&mut self) {
        self.items = self.items.iter().map(|item| item / 3).collect();
    }

    fn throw_a(&mut self, monkeys: &Vec<RefCell<Monkey>>) {
        for item in &self.items {
            let throw_to = if item % self.test_val as u64 == 0 {
                self.throw_to.0
            } else {
                self.throw_to.1
            };

            monkeys[throw_to as usize].borrow_mut().items.push(*item);
        }
        self.items.clear();
    }

    fn throw_b(&mut self, monkeys: &Vec<RefCell<Monkey>>, modulo: u64) {
        for item in &self.items {
            let throw_to = if item % self.test_val as u64 == 0 {
                self.throw_to.0
            } else {
                self.throw_to.1
            };

            monkeys[throw_to as usize]
                .borrow_mut()
                .items
                .push(*item % modulo);
        }
        self.items.clear();
    }
}

impl From<&[String]> for Monkey {
    fn from(lines: &[String]) -> Self {
        let items = lines[1]
            .trim()
            .split(" ")
            .skip(2)
            .map(|num_with_comma| {
                if num_with_comma.ends_with(",") {
                    num_with_comma.strip_suffix(",").unwrap().parse().unwrap()
                } else {
                    num_with_comma.parse().unwrap()
                }
            })
            .collect();

        let operation: Vec<&str> = lines[2].trim().split(" ").skip(4).collect();
        let op_fn = OpFn::from_str(operation[0]).unwrap();
        let op_val: OpVal = OpVal::from_str(operation[1]).unwrap();

        let test_val: u32 = lines[3].trim().split(" ").last().unwrap().parse().unwrap();

        let throw_1: u32 = lines[4].trim().split(" ").last().unwrap().parse().unwrap();
        let throw_2: u32 = lines[5].trim().split(" ").last().unwrap().parse().unwrap();
        Monkey {
            items,
            op_val,
            op_fn,
            test_val,
            throw_to: (throw_1, throw_2),
            inspected: 0,
        }
    }
}

fn part_a() {
    // chunky monkeys!
    let monkeys: Vec<RefCell<Monkey>> = read_lines("./day_11/input.txt")
        .filter(|line| line != "")
        .collect::<Vec<String>>()
        .chunks(6)
        .map(|chunk| RefCell::new(chunk.into()))
        .collect();

    for _ in 0..20 {
        for monkey in &monkeys {
            // update the items
            monkey.borrow_mut().take_turn_a(&monkeys);
        }
    }

    let mut inspected_nums = monkeys
        .iter()
        .map(|monkey| monkey.borrow().inspected)
        .collect::<Vec<u32>>();

    inspected_nums.sort_by(|a, b| b.cmp(&a));
    let business = inspected_nums[0] * inspected_nums[1];
    println!("{}", business);
}

fn part_b() {
    // chunky monkeys!
    let monkeys: Vec<RefCell<Monkey>> = read_lines("./day_11/input.txt")
        .filter(|line| line != "")
        .collect::<Vec<String>>()
        .chunks(6)
        .map(|chunk| RefCell::new(chunk.into()))
        .collect();

    let lowest_common_mult = monkeys
        .iter()
        .map(|monkey| monkey.borrow().test_val)
        .fold(1, |prod, val| prod * val);

    for _ in 0..10000 {
        for monkey in &monkeys {
            // update the items
            monkey
                .borrow_mut()
                .take_turn_b(&monkeys, lowest_common_mult as u64);
        }
    }

    let mut inspected_nums: Vec<u64> = monkeys
        .iter()
        .map(|monkey| monkey.borrow().inspected as u64)
        .collect();

    inspected_nums.sort_by(|a, b| b.cmp(&a));
    let business = inspected_nums[0] * inspected_nums[1];
    println!("{}", business);
}

fn main() {
    part_a();
    part_b();
}
