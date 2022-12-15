use std::str::FromStr;

use utils::{abs, read_lines};

enum Instr {
    Noop,
    Addx(i32),
}

impl FromStr for Instr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        match parts[0] {
            "noop" => Ok(Instr::Noop),
            "addx" => Ok(Instr::Addx(parts[1].parse().unwrap())),
            _ => Err(String::from("cannot parse instruction")),
        }
    }
}

fn part_a() {
    let mut x = 1;
    let mut clock = 1;

    let mut strengths: Vec<i32> = Vec::new();

    let incr_clock = |x: i32, clock: &mut i32, strengths: &mut Vec<i32>| {
        if *clock <= 220 && *clock % 40 == 20 {
            strengths.push(x * *clock);
        }
        *clock += 1;
    };

    read_lines("./day_10/input.txt")
        .map(|line| Instr::from_str(&line).unwrap())
        .for_each(|instr| match instr {
            Instr::Addx(val) => {
                incr_clock(x, &mut clock, &mut strengths);
                incr_clock(x, &mut clock, &mut strengths);
                x += val;
            }
            _ => incr_clock(x, &mut clock, &mut strengths),
        });

    let strength_sum: i32 = strengths.iter().sum();
    println!("{}", strength_sum);
}

fn part_b() {
    let mut x = 1;
    let mut clock: usize = 0;

    let mut crt: Vec<Vec<bool>> = Vec::new();

    let incr_clock = |x: i32, clock: &mut usize, crt: &mut Vec<Vec<bool>>| {
        let row = *clock / 40;
        let col = *clock % 40;

        if col == 0 {
            crt.push(Vec::new());
        }

        crt[row].push(abs(col as i32 - x) <= 1);

        *clock += 1;
    };

    read_lines("./day_10/input.txt")
        .map(|line| Instr::from_str(&line).unwrap())
        .for_each(|instr| match instr {
            Instr::Addx(val) => {
                incr_clock(x, &mut clock, &mut crt);
                incr_clock(x, &mut clock, &mut crt);
                x += val;
            }
            _ => incr_clock(x, &mut clock, &mut crt),
        });

    crt.iter().for_each(|row| {
        row.iter()
            .for_each(|sprite| print!("{}", if *sprite { "#" } else { "." }));
        println!();
    });
}

fn main() {
    part_a();
    part_b();
}
