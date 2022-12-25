use std::collections::{HashMap, VecDeque};

use utils::read_lines;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    time: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: u32,
    geode_robot: u32,
}

impl State {
    fn new(
        time: u32,
        ore: u32,
        clay: u32,
        obsidian: u32,
        geode: u32,
        ore_robot: u32,
        clay_robot: u32,
        obsidian_robot: u32,
        geode_robot: u32,
    ) -> Self {
        State {
            time,
            ore,
            clay,
            obsidian,
            geode,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_recipe: u32,             // ore
    clay_recipe: u32,            // ore
    obsidian_recipe: (u32, u32), // ore, clay
    geode_recipe: (u32, u32),    // ore, obsidian
}

impl Blueprint {
    fn max_yield(&self, time: u32) -> u32 {
        let mut queue: VecDeque<State> = VecDeque::new();
        let mut max_geodes = 0;

        let start = State::new(time, 0, 0, 0, 0, 1, 0, 0, 0);
        queue.push_back(start);

        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            if cur.geode > max_geodes {
                max_geodes = cur.geode;
            }

            if cur.time >= 1 {
                let next = State {
                    time: cur.time - 1,
                    ore: cur.ore + cur.ore_robot,
                    clay: cur.clay + cur.clay_robot,
                    obsidian: cur.obsidian + cur.obsidian_robot,
                    geode: cur.geode + cur.geode_robot,
                    ..cur
                };

                // try making geode robot
                if cur.ore >= self.geode_recipe.0 && cur.obsidian >= self.geode_recipe.1 {
                    let mut next = next.clone();
                    next.ore -= self.geode_recipe.0;
                    next.obsidian -= self.geode_recipe.1;
                    next.geode_robot += 1;
                    queue.push_back(next);

                    continue;
                }

                // try making obsidian robot
                if cur.obsidian_robot < self.geode_recipe.1
                    && cur.ore >= self.obsidian_recipe.0
                    && cur.clay >= self.obsidian_recipe.1
                {
                    let mut next = next.clone();
                    next.ore -= self.obsidian_recipe.0;
                    next.clay -= self.obsidian_recipe.1;
                    next.obsidian_robot += 1;
                    queue.push_back(next);

                    if cur.obsidian_robot == 0 {
                        continue;
                    }
                }

                // try making clay robot
                if cur.clay_robot < self.obsidian_recipe.1 && cur.ore >= self.clay_recipe {
                    let mut next = next.clone();
                    next.ore -= self.clay_recipe;
                    next.clay_robot += 1;
                    queue.push_back(next);
                }

                // try making ore robot
                if cur.ore_robot < self.clay_recipe && cur.ore >= self.ore_recipe {
                    let mut next = next.clone();
                    next.ore -= self.ore_recipe;
                    next.ore_robot += 1;
                    queue.push_back(next);
                }

                queue.push_back(next);
            }
        }

        max_geodes
    }
}

impl From<String> for Blueprint {
    fn from(s: String) -> Self {
        let parts: Vec<&str> = s.split(' ').collect();

        let id = parts[1].strip_suffix(':').unwrap().parse().unwrap();
        let ore_recipe = parts[6].parse().unwrap();
        let clay_recipe = parts[12].parse().unwrap();
        let obsidian_recipe = (parts[18].parse().unwrap(), parts[21].parse().unwrap());
        let geode_recipe = (parts[27].parse().unwrap(), parts[30].parse().unwrap());

        Blueprint {
            id,
            ore_recipe,
            clay_recipe,
            obsidian_recipe,
            geode_recipe,
        }
    }
}

fn part_a() {
    let quality_sum: u32 = read_lines("./day_19/input.txt")
        .map(|line| {
            let blueprint: Blueprint = line.into();
            println!("{}", blueprint.id);
            blueprint.max_yield(24) * blueprint.id
        })
        .sum();

    println!("{quality_sum}");
}

fn part_b() {
    let quality_product: u32 = read_lines("./day_19/input.txt")
        .take(3)
        .map(|line| {
            let blueprint: Blueprint = line.into();
            println!("{}", blueprint.id);
            blueprint.max_yield(32)
        })
        .product();
    println!("{quality_product}");
}

fn main() {
    part_a();
    // part_b();
}
