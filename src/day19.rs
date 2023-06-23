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
        


        ("".to_owned(), 0)
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

struct Simulation {}

pub fn process(input: String) {
    let result = part1(input);
    println!("Result: {}", result);
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
