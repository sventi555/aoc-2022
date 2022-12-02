use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Missing input file");

    let parsed_input = contents.split("\n");

    // Part A (X = rock, Y = paper, Z = scissors)
    let score: i32 = parsed_input
        .clone()
        .map(|game| match game {
            "A X" => 4, // rock v rock
            "B X" => 1, // rock v paper
            "C X" => 7, // rock v scissors
            "A Y" => 8, // paper v rock
            "B Y" => 5, // paper v paper
            "C Y" => 2, // paper v scissors
            "A Z" => 3, // scissors v rock
            "B Z" => 9, // scissors v paper
            "C Z" => 6, // scissors v scissors
            _ => 0,
        })
        .sum();

    println!("{}", score);

    // Part B (X = lose, Y = draw, Z = win)
    let score: i32 = parsed_input
        .clone()
        .map(|game| match game {
            "A X" => 3, // scissors v rock
            "B X" => 1, // rock v paper
            "C X" => 2, // paper v scissors
            "A Y" => 4, // rock v rock
            "B Y" => 5, // paper v paper
            "C Y" => 6, // scissors v scissors
            "A Z" => 8, // paper v rock
            "B Z" => 9, // scissors v paper
            "C Z" => 7, // rock v scissors
            _ => 0,
        })
        .sum();

    println!("{}", score);
}
