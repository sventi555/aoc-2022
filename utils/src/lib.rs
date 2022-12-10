use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|line| line.unwrap())
}

pub fn abs(val: i32) -> i32 {
    if val < 0 {
        -val
    } else {
        val
    }
}
