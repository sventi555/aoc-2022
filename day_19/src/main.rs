use std::collections::{HashMap, VecDeque};

use utils::read_lines;

fn geode_limit(geodes: u32, rate: u32, time: u32) -> u32 {
    let mut cur = geodes;
    let mut rate = rate;
    for _ in 0..time {
        cur += rate;
        rate += 1;
    }

    cur
}

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
    skipped: bool,
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
        skipped: bool,
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
            skipped,
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
    // the REAL big time saver here is `skipped_over_before` because it prunes
    // worse performing permutations early on. The other time savers
    // - checking the max geodes in best case scenario
    // - not building more robots than are needed for most expensive recipe
    // - not skipping if we are able to build at least one robot
    // help out a bit, but not NEARLY as much as the `skipped_over_before` trick
    //
    // Some pruning strategies inspired by this comment - https://www.reddit.com/r/adventofcode/comments/zs4m2w/comment/j168tiq/?utm_source=share&utm_medium=web2x&context=3
    fn max_yield(&self, time: u32) -> u32 {
        let mut queue: VecDeque<State> = VecDeque::new();
        let mut max_geodes = 0;

        let start = State::new(time, 0, 0, 0, 0, 1, 0, 0, 0, false);
        queue.push_back(start);

        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            if cur.geode > max_geodes {
                max_geodes = cur.geode;
            }

            // end early if we can't beat our record in the absolute best scenario
            if cur.time >= 1 && geode_limit(cur.geode, cur.geode_robot, cur.time) > max_geodes {
                let mut next = State {
                    time: cur.time - 1,
                    ore: cur.ore + cur.ore_robot,
                    clay: cur.clay + cur.clay_robot,
                    obsidian: cur.obsidian + cur.obsidian_robot,
                    geode: cur.geode + cur.geode_robot,
                    skipped: false,
                    ..cur
                };

                // ALWAYS make a geode robot if able
                if cur.ore >= self.geode_recipe.0 && cur.obsidian >= self.geode_recipe.1 {
                    let mut next = next.clone();
                    next.ore -= self.geode_recipe.0;
                    next.obsidian -= self.geode_recipe.1;
                    next.geode_robot += 1;
                    queue.push_back(next);

                    continue;
                }

                // try making obsidian robot
                let skipped_over_before = cur.skipped
                    && cur.ore - cur.ore_robot >= self.obsidian_recipe.0
                    && cur.clay - cur.clay_robot >= self.obsidian_recipe.1;
                if !skipped_over_before
                    && cur.obsidian_robot < self.geode_recipe.1
                    && cur.ore >= self.obsidian_recipe.0
                    && cur.clay >= self.obsidian_recipe.1
                {
                    let mut next = next.clone();
                    next.ore -= self.obsidian_recipe.0;
                    next.clay -= self.obsidian_recipe.1;
                    next.obsidian_robot += 1;
                    queue.push_back(next);
                }

                // try making clay robot
                let skipped_over_before =
                    cur.skipped && cur.ore - cur.ore_robot >= self.clay_recipe;
                if !skipped_over_before
                    && cur.clay_robot < self.obsidian_recipe.1
                    && cur.ore >= self.clay_recipe
                {
                    let mut next = next.clone();
                    next.ore -= self.clay_recipe;
                    next.clay_robot += 1;
                    queue.push_back(next);
                }

                // try making ore robot
                let skipped_over_before = cur.skipped && cur.ore - cur.ore_robot >= self.ore_recipe;
                if !skipped_over_before
                    && cur.time > self.ore_recipe
                    && cur.ore_robot < self.clay_recipe
                    && cur.ore >= self.ore_recipe
                {
                    let mut next = next.clone();
                    next.ore -= self.ore_recipe;
                    next.ore_robot += 1;
                    queue.push_back(next);
                }

                if cur.ore < self.ore_recipe
                    || cur.ore < self.clay_recipe
                    || cur.ore < self.obsidian_recipe.0
                    || cur.clay < self.obsidian_recipe.1
                    || cur.ore < self.geode_recipe.0
                    || cur.obsidian < self.geode_recipe.1
                {
                    next.skipped = true;
                    queue.push_back(next);
                }
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
            let max_yield = blueprint.max_yield(32);
            max_yield
        })
        .product();
    println!("{quality_product}");
}

fn main() {
    part_a();
    part_b();
}
