use std::{num::ParseIntError, str::FromStr};

use utils::read_lines;

struct Assignment {
    start: i32,
    end: i32,
}

impl FromStr for Assignment {
    type Err = ParseAssignmentPairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let endpoints: Vec<&str> = s.split("-").collect();
        if endpoints.len() != 2 {
            return Err(ParseAssignmentPairError {});
        }
        Ok(Assignment {
            start: endpoints[0].parse()?,
            end: endpoints[1].parse()?,
        })
    }
}

impl Assignment {
    fn size(&self) -> i32 {
        self.end - self.start
    }
}

struct AssignmentPair {
    left: Assignment,
    right: Assignment,
}

#[derive(Debug)]
struct ParseAssignmentPairError {}

impl From<ParseIntError> for ParseAssignmentPairError {
    fn from(_: ParseIntError) -> Self {
        ParseAssignmentPairError {}
    }
}

impl FromStr for AssignmentPair {
    type Err = ParseAssignmentPairError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let assignments: Vec<&str> = s.split(",").collect();
        if assignments.len() != 2 {
            return Err(ParseAssignmentPairError {});
        }
        Ok(AssignmentPair {
            left: Assignment::from_str(assignments[0])?,
            right: Assignment::from_str(assignments[1])?,
        })
    }
}

impl AssignmentPair {
    fn fully_contains_other(&self) -> bool {
        let (smaller, larger) = if self.left.size() <= self.right.size() {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        };

        smaller.start >= larger.start && smaller.end <= larger.end
    }

    fn partially_contains_other(&self) -> bool {
        let (smaller, larger) = if self.left.size() <= self.right.size() {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        };

        smaller.start >= larger.start && smaller.start <= larger.end
            || smaller.end >= larger.start && smaller.end <= larger.end
    }
}

fn part_a() {
    let overlapping_assignments = read_lines("./day_4/input.txt").fold(0, |count, assignments| {
        let assignment_pair = AssignmentPair::from_str(&assignments).unwrap();
        count
            + if assignment_pair.fully_contains_other() {
                1
            } else {
                0
            }
    });

    println!("{}", overlapping_assignments);
}

fn part_b() {
    let overlapping_assignments = read_lines("./day_4/input.txt").fold(0, |count, assignments| {
        let assignment_pair = AssignmentPair::from_str(&assignments).unwrap();
        count
            + if assignment_pair.partially_contains_other() {
                1
            } else {
                0
            }
    });

    println!("{}", overlapping_assignments);
}

fn main() {
    part_a();
    part_b();
}
