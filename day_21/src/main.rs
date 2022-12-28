use core::panic;
use std::collections::HashMap;

use utils::read_lines;

enum Op {
    Add,
    Sub,
    Mult,
    Div,
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mult,
            "/" => Op::Div,
            _ => panic!(),
        }
    }
}

enum Yell {
    Expression { left: String, right: String, op: Op },
    Val(i64),
}

fn has_humn(name: &String, monkeys: &HashMap<String, Yell>) -> bool {
    let yell = &monkeys[name];

    match yell {
        Yell::Expression { left, right, .. } => {
            if left == "humn" || right == "humn" {
                return true;
            } else {
                return has_humn(&left, monkeys) || has_humn(&right, monkeys);
            }
        }
        Yell::Val(_) => false,
    }
}

fn monkey_val(name: &String, monkeys: &HashMap<String, Yell>, humn_val: i64) -> i64 {
    if name == "humn" {
        return humn_val;
    }
    let yell = &monkeys[name];

    match yell {
        Yell::Val(n) => *n,
        Yell::Expression { left, right, op } => match op {
            Op::Add => monkey_val(&left, monkeys, humn_val) + monkey_val(&right, monkeys, humn_val),
            Op::Sub => monkey_val(&left, monkeys, humn_val) - monkey_val(&right, monkeys, humn_val),
            Op::Mult => {
                monkey_val(&left, monkeys, humn_val) * monkey_val(&right, monkeys, humn_val)
            }
            Op::Div => monkey_val(&left, monkeys, humn_val) / monkey_val(&right, monkeys, humn_val),
        },
    }
}

fn part_a() {
    let monkeys: HashMap<String, Yell> = read_lines("./day_21/input.txt")
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();

            let name = parts[0].strip_suffix(':').unwrap().to_string();

            if parts[1].parse::<i64>().is_ok() {
                (name, Yell::Val(parts[1].parse().unwrap()))
            } else {
                (
                    name,
                    Yell::Expression {
                        left: parts[1].to_string(),
                        right: parts[3].to_string(),
                        op: parts[2].into(),
                    },
                )
            }
        })
        .collect();

    let total = monkey_val(&"root".to_string(), &monkeys, 4977);
    println!("{total}");
}

fn part_b() {
    let monkeys: HashMap<String, Yell> = read_lines("./day_21/input.txt")
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();

            let name = parts[0].strip_suffix(':').unwrap().to_string();

            if parts[1].parse::<i64>().is_ok() {
                (name, Yell::Val(parts[1].parse().unwrap()))
            } else {
                (
                    name,
                    Yell::Expression {
                        left: parts[1].to_string(),
                        right: parts[3].to_string(),
                        op: parts[2].into(),
                    },
                )
            }
        })
        .collect();

    let left = "rnsd";
    let right = "vlzj";
    let (side_with_humn, side_without_humn) = if has_humn(&left.to_string(), &monkeys) {
        (left.to_string(), right.to_string())
    } else {
        (right.to_string(), left.to_string())
    };

    let mut cur_humn = 0;
    let static_val = monkey_val(&side_without_humn, &monkeys, 0);
    let mut humn_side_val = monkey_val(&side_with_humn, &monkeys, cur_humn);
    let mut comp = static_val.cmp(&humn_side_val);

    let mut cur_interval = 1000000000;
    loop {
        humn_side_val = monkey_val(&side_with_humn, &monkeys, cur_humn);

        if static_val == humn_side_val {
            cur_humn += cur_interval;
            break;
        }

        let new_comp = static_val.cmp(&humn_side_val);
        if new_comp != comp {
            comp = new_comp;
            cur_interval /= -2;
        }

        cur_humn += cur_interval;
    }

    // there are multiple correct solutions with integer division!
    // I did some manual manipulation to get the "correct" one for advent,
    // but I think this implementation is good with that assumption in place.
    println!("{cur_humn}");
}
fn main() {
    part_a();
    part_b();
}
