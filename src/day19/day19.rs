use regex::Regex;
use std::{
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

    pub fn score(&self, time_limit: u8) -> u32 {
        let mut max_score = 0;

        let max_robots = self
            .costs
            .values()
            .map(|a| *a)
            .reduce(|a, b| Resources {
                ore: a.ore.max(b.ore),
                clay: a.clay.max(b.clay),
                obsidian: a.obsidian.max(b.obsidian),
                geode: a.geode.max(b.geode),
            })
            .unwrap();

        self.score_rec(
            time_limit,
            &mut max_score,
            &max_robots,
            1,
            Resources {
                ore: 1,
                ..Resources::default()
            },
            Resources::default(),
        )
    }

    fn score_rec(
        &self,
        time_limit: u8,
        max_score: &mut u32,
        max_robots: &Resources,
        minute: u8,
        robots: Resources,
        storage: Resources,
    ) -> u32 {
        Robot::unlocked(robots)
            .iter()
            .map(|robot| {
                match robot {
                    // No point in making more robots than max
                    Robot::Ore if robots.ore >= max_robots.ore => return 0,
                    Robot::Clay if robots.clay >= max_robots.clay => return 0,
                    Robot::Obsidian if robots.obsidian >= max_robots.obsidian => return 0,
                    _ => (),
                }

                let robot_cost = self.costs.get(&robot).unwrap();
                let mut next_minute = minute;
                let mut next_storage = storage;

                while next_minute <= time_limit && !next_storage.has(robot_cost) {
                    next_minute = next_minute + 1;
                    next_storage = next_storage + robots;
                }

                if next_minute >= time_limit {
                    if next_storage.geode > *max_score {
                        *max_score = next_storage.geode;
                        println!("{}", next_storage.geode)
                    }
                    return next_storage.geode;
                } else {
                    next_minute = next_minute + 1;
                    next_storage = next_storage - *robot_cost + robots;
                    let next_robots = Resources {
                        ore: robots.ore + if let Robot::Ore = robot { 1 } else { 0 },
                        clay: robots.clay + if let Robot::Clay = robot { 1 } else { 0 },
                        obsidian: robots.obsidian + if let Robot::Obsidian = robot { 1 } else { 0 },
                        geode: robots.geode + if let Robot::Geode = robot { 1 } else { 0 },
                    };
                    return self.score_rec(
                        time_limit,
                        max_score,
                        max_robots,
                        next_minute,
                        next_robots,
                        next_storage,
                    );
                }
            })
            .max()
            .unwrap()
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Robot {
    const ALL: [Robot; 4] = [Robot::Geode, Robot::Obsidian, Robot::Clay, Robot::Ore];

    pub fn unlocked(robots: Resources) -> &'static [Robot] {
        if robots.obsidian > 0 {
            &Robot::ALL[0..4]
        } else if robots.clay > 0 {
            &Robot::ALL[1..4]
        } else {
            &Robot::ALL[2..4]
        }
    }
}

pub fn process(input: String) {
    let result = part2(input);

    println!("Result: {}", result);
}

fn single_bp(input: String) -> u32 {
    let bps = Blueprint::parse(&input);
    let bp = bps.get(2).unwrap();
    let result = bp.score(32);

    return result;
}

fn part1(input: String) -> String {
    let bps = Blueprint::parse(&input);

    let total_score: u32 = bps
        .iter()
        .map(|bp| {
            let score = bp.score(24);
            println!("Blueprint {}: {} geodes", bp.id, score);
            bp.id * score
        })
        .sum();

    return format!("Total score: {total_score}");
}

fn part2(input: String) -> String {
    let bps = Blueprint::parse(&input);

    let total_score: u32 = bps
        .iter()
        .take(3)
        .map(|bp| {
            let score = bp.score(32);
            println!("Blueprint {}: {} geodes", bp.id, score);
            score
        })
        .reduce(|a, b| a * b)
        .unwrap();

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

    #[test]
    fn test_part1() {
        let input = "".to_owned();
        let result = part1(input);
        assert_eq!(result, "Total score: 1589");
    }

    #[test]
    fn test_part2() {
        let input = "".to_owned();
        let result = part2(input);
        assert_eq!(result, "Total score: 29348");
    }
    
}
