use rayon::prelude::*;
use regex::Regex;
use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Resources {
    pub fn has(&self, other: &Resources) -> bool {
        return self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode;
    }
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

    pub fn score(&self) -> u32 {
        self.score_rec(
            1,
            Resources {
                ore: 1,
                ..Resources::default()
            },
            Resources::default(),
        )
    }

    fn score_rec(&self, minute: u8, robots: Resources, storage: Resources) -> u32 {
        Robot::unlocked(robots)
            .iter()
            .map(|next_robot| {
                let robot_cost = self.costs.get(&next_robot).unwrap();
                let mut next_minute = minute;
                let mut next_storage = storage;

                while next_minute < 25 && !next_storage.has(robot_cost) {
                    next_minute = next_minute + 1;
                    next_storage = next_storage + robots;
                    //println!("minute: {}", next_minute);
                }

                if next_minute >= 24 {
                    return next_storage.geode;
                } else {
                    next_minute = next_minute + 1;
                    next_storage = next_storage - *robot_cost + robots;
                    let next_robots = Resources {
                        ore: robots.ore + if let Robot::Ore = next_robot { 1 } else { 0 },
                        clay: robots.clay + if let Robot::Clay = next_robot { 1 } else { 0 },
                        obsidian: robots.obsidian
                            + if let Robot::Obsidian = next_robot {
                                1
                            } else {
                                0
                            },
                        geode: robots.geode + if let Robot::Geode = next_robot { 1 } else { 0 },
                    };
                    return self.score_rec(next_minute, next_robots, next_storage);
                }
            })
            .max()
            .unwrap()
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
    pub fn all() -> [Robot; 4] {
        [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode]
    }

    pub fn unlocked(robots: Resources) -> Vec<Robot> {
        let mut result = vec![Robot::Ore, Robot::Clay];
        if robots.clay > 0 {
            result.push(Robot::Obsidian);
        }
        if robots.obsidian > 0 {
            result.push(Robot::Geode);
        }
        result
    }
}

pub fn process(input: String) {
    let result = bp1_test(input);

    println!("Result: {}", result);
}

fn bp1_test(input: String) -> u32 {
    let bps = Blueprint::parse(&input);
    let bp = bps.get(1).unwrap();
    let result = bp.score();

    return result;
}

fn part1(input: String) -> String {
    let bps = Blueprint::parse(&input);

    let total_score: u32 = bps
        .par_iter()
        .map(|bp| {
            let score = bp.score();
            println!("Blueprint {}: {} geodes", bp.id, score);
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
