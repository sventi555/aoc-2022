use std::{collections::HashMap, str::FromStr};

use utils::read_lines;

enum Command {
    File(u32),
    Dir(String),
    Cd(String),
    Ls,
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        match parts[0] {
            "$" => match parts[1] {
                "cd" => Ok(Command::Cd(String::from(parts[2]))),
                "ls" => Ok(Command::Ls),
                _ => Err(String::from("invalid command")),
            },
            "dir" => Ok(Command::Dir(String::from(parts[1]))),
            size => {
                if let Ok(file_size) = size.parse::<u32>() {
                    Ok(Command::File(file_size))
                } else {
                    Err(String::from("invalid file size"))
                }
            }
        }
    }
}

struct Dir {
    size: u32,
    children: HashMap<String, Dir>,
}

impl Dir {
    fn new() -> Self {
        Dir {
            size: 0,
            children: HashMap::new(),
        }
    }
}

impl From<&Vec<String>> for Dir {
    fn from(input: &Vec<String>) -> Self {
        let mut dir = Dir::new();

        fn build_dir(dir: &mut Dir, input: &Vec<String>, input_index: &mut usize) -> u32 {
            while let Some(line) = input.get(*input_index) {
                *input_index += 1;
                let cmd: Command = line.parse().unwrap();
                match cmd {
                    Command::Cd(path) => {
                        if path == ".." {
                            return dir.size;
                        } else {
                            dir.size +=
                                build_dir(dir.children.get_mut(&path).unwrap(), input, input_index);
                        }
                    }
                    Command::Dir(name) => {
                        dir.children.insert(name, Dir::new());
                    }
                    Command::File(file_size) => {
                        dir.size += file_size;
                    }
                    _ => {}
                }
            }

            dir.size
        }

        build_dir(&mut dir, input, &mut 1);
        dir
    }
}

struct DirIter<'a> {
    stack: Vec<&'a Dir>,
}

impl<'a> IntoIterator for &'a Dir {
    type Item = &'a Dir;

    type IntoIter = DirIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DirIter { stack: vec![self] }
    }
}

impl<'a> Iterator for DirIter<'a> {
    type Item = &'a Dir;

    fn next(&mut self) -> Option<Self::Item> {
        let next_dir = self.stack.pop()?;
        self.stack.extend(next_dir.children.values());
        Some(next_dir)
    }
}

fn part_a() {
    let lines: Vec<String> = read_lines("./day_7/input.txt").collect();
    let root = Dir::from(&lines);

    let total: u32 = root
        .into_iter()
        .map(|dir| dir.size)
        .filter(|size| size <= &100000)
        .sum();
    println!("{}", total);
}

fn part_b() {
    let lines: Vec<String> = read_lines("./day_7/input.txt").collect();
    let root = Dir::from(&lines);

    let root_size = root.size;
    let needed_space = 30000000 - (70000000 - root_size);

    let smallest_to_delete: u32 = root
        .into_iter()
        .map(|dir| dir.size)
        .filter(|size| size >= &needed_space)
        .min()
        .unwrap();

    println!("{}", smallest_to_delete);
}

fn main() {
    part_a();
    part_b();
}
