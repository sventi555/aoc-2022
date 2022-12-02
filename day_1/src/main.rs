use std::fs;

fn main() {
    let contents = fs::read_to_string("./day_1/input.txt").expect("Missing input file");
    let parsed_input = contents.split("\n\n").map(|group| {
        group
            .split("\n")
            .map(|num| num.parse::<i32>().unwrap_or(0))
            .sum::<i32>()
    });

    // part A
    let max_cal = parsed_input.clone().max().unwrap();

    println!("{}", max_cal);

    // part B
    let mut cal_counts: Vec<i32> = parsed_input.collect();
    cal_counts.sort();

    let len = cal_counts.len();
    let max_3_cal: i32 = cal_counts[(len - 3)..len].iter().sum();

    println!("{}", max_3_cal);
}
