use rayon::prelude::*;
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::format,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

impl Add for Resources {
    type Output = Resources;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Sub for Resources {
    type Output = Resources;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl PartialEq for Resources {
    fn eq(&self, other: &Self) -> bool {
        self.ore == other.ore
            && self.clay == other.clay
            && self.obsidian == other.obsidian
            && self.geode == other.geode
    }
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.eq(other) {
            Some(Ordering::Equal)
        } else if self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
        {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    costs: HashMap<Robot, Resources>,
}

impl Blueprint {
    pub fn parse(input: &str) -> Vec<Blueprint> {
        let pattern = Regex::new(
            r"Blueprint (\d+):\s+Each ore robot costs (\d+) ore\.\s+Each clay robot costs (\d+) ore\.\s+Each obsidian robot costs (\d+) ore and (\d+) clay\.\s+Each geode robot costs (\d+) ore and (\d+) obsidian\.",
        ).unwrap();

        let blueprints = pattern
            .captures_iter(input)
            .filter_map(|c| {
                Some(Blueprint {
                    id: c.get(1)?.as_str().parse::<u32>().ok()?,
                    costs: HashMap::from([
                        (
                            Robot::Ore,
                            Resources {
                                ore: c.get(2)?.as_str().parse::<u32>().ok()?,
                                ..Resources::default()
                            },
                        ),
                        (
                            Robot::Clay,
                            Resources {
                                ore: c.get(3)?.as_str().parse::<u32>().ok()?,
                                ..Resources::default()
                            },
                        ),
                        (
                            Robot::Obsidian,
                            Resources {
                                ore: c.get(4)?.as_str().parse::<u32>().ok()?,
                                clay: c.get(5)?.as_str().parse::<u32>().ok()?,
                                ..Resources::default()
                            },
                        ),
                        (
                            Robot::Geode,
                            Resources {
                                ore: c.get(6)?.as_str().parse::<u32>().ok()?,
                                obsidian: c.get(7)?.as_str().parse::<u32>().ok()?,
                                ..Resources::default()
                            },
                        ),
                    ]),
                })
            })
            .collect::<Vec<_>>();
        blueprints
    }

    pub fn score(&self) -> (String, u32) {
        self.score_rec(
            1,
            Resources {
                ore: 1,
                ..Resources::default()
            },
            Resources::default(),
        )
    }

    fn score_rec(&self, minute: u8, robots: Resources, storage: Resources) -> (String, u32) {
        let mut log = format!("== Minute {minute} ==\n");
        let next_minute = minute + 1;
        let next_storage = storage + robots;

        // Prune
        if minute >= 10 && robots.clay == 0 {
            return (log + "cut no clay production", 0);
        } else if minute >= 15 && robots.obsidian == 0 {
            return (log + "cut no obsidian production", 0);
        } else if minute >= 20 && robots.geode == 0 {
            return (log + "cut no geode production", 0);
        } else if minute >= 24 {
            println!("tick: {}", next_storage.geode);
            return (log, next_storage.geode);
        }

        // Spend
        let mut best_robot = None;
        let (mut best_log, mut best_score) = self.score_rec(next_minute, robots, next_storage);

        for robot in Robot::options() {
            let robot_cost = self.costs.get(&robot).unwrap();
            if storage > *robot_cost {
                let next_robots = Resources {
                    ore: robots.ore + if let Robot::Ore = robot { 1 } else { 0 },
                    clay: robots.clay + if let Robot::Clay = robot { 1 } else { 0 },
                    obsidian: robots.obsidian + if let Robot::Obsidian = robot { 1 } else { 0 },
                    geode: robots.geode + if let Robot::Geode = robot { 1 } else { 0 },
                };

                let branch = self.score_rec(next_minute, next_robots, next_storage);

                if branch.1 > best_score {
                    best_log = branch.0;
                    best_score = branch.1;
                    best_robot = Some(robot);
                }
            }
        }
        if let Some(r) = &best_robot {
            log.push_str(&format!("Started building {:?} robot", r));
        }
        // Collect
        log.push_str(&format!("Collected resources: {:?}", next_storage));

        // Robot ready
        if let Some(r) = &best_robot {
            log.push_str(&format!(
                "Finished building a {:?} robot, now we have {:?}",
                r, robots
            ));
        }
        return (log + &best_log, best_score);
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Robot {
    pub fn options() -> [Robot; 4] {
        [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode]
    }
}

struct Simulation {}

pub fn process(input: String) {
    let result = bp1_test(input);

    println!("Result: {}", result);
}

fn bp1_test(input: String) -> u32 {
    let bps = Blueprint::parse(&input);
    let bp = bps.get(0).unwrap();
    let (log, result) = bp.score();

    println!("{}", log);
    return result;
}

fn part1(input: String) -> String {
    let bps = Blueprint::parse(&input);

    let total_score: u32 = bps
        .par_iter()
        .map(|bp| {
            let (recipe, score) = bp.score();
            println!("Blueprint {}: {} geodes", bp.id, score);
            println!("{}", recipe);
            bp.id * score
        })
        .sum();

    return format!("Total score: {total_score}");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse() {
        let input = "";
        let bp = Blueprint::parse(input);
        println!("{:?}", bp);
    }
}
