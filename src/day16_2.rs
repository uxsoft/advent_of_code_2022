use std::collections::{BTreeMap, HashSet, LinkedList};
use std::time::Instant;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag};
use nom::character::complete::{alpha1, newline};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use rayon::prelude::*;

#[derive(Debug)]
struct Room {
    name: String,
    flow: u32,
    tunnels: Vec<String>
}

impl Room {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    pub fn parse(input: &str) -> IResult<&str, Room> {
        map(tuple((
            preceded(tag("Valve "), alpha1),
            preceded(tag(" has flow rate="), nom::character::complete::u32),
            preceded(alt((tag("; tunnels lead to valves "), tag("; tunnel leads to valve "))), 
                     separated_list1(tag(", "), alpha1)),
        )), |(a, f, t)| Room {
            name: String::from(a),
            flow: f,
            tunnels: t.iter().map(ToString::to_string).collect_vec(),
        })(input)
    }
}

#[derive(Debug)]
struct Cave {
    rooms: Vec<Room>,
    edges: Vec<(usize, usize)>,
    distances: Vec<Vec<u32>>,
    names: BTreeMap<String, usize>,
    flow_rates: Vec<u32>,
    working_valves: Vec<usize>
}

struct Score {
    max: u32
}

impl Score {
    pub fn publish_result(&mut self, plan: &Plan)
    {
        let score = plan.player.total_flow + plan.elephant.total_flow;
        if self.max < score {
            self.max = score;
            println!("New max: {}", score);   
        }
    }
}

impl Cave {
    pub fn new(rooms: Vec<Room>) -> Self {
        let mut cave = Cave {
            rooms,
            edges: vec![],
            distances: vec![],
            names: BTreeMap::new(),
            working_valves: vec![],
            flow_rates: vec![]
        };
        
        cave.working_valves = cave.build_working_valves();
        cave.flow_rates = cave.build_flow_rates();
        cave.names = cave.build_names();
        cave.edges = cave.build_edges();
        cave.distances = cave.build_distances_flw();
        cave
    }
    
    pub fn parse(input: &str) -> IResult<&str, Cave> {
        map(
            separated_list1(newline, Room::parse),
            Cave::new)(input)
    }
    
    pub fn get_by_name(&self, name: &str) -> &Room {
        self.rooms.get(self.index_of(name)).unwrap()
    }
    
    pub fn index_of(&self, name: &str) -> usize {
        self.names.get(name).unwrap().clone()
    }
    
    fn build_names(&self) -> BTreeMap<String, usize> {
        self.rooms
            .iter()
            .enumerate()
            .map(|(i, r)| (r.name.clone(), i))
            .collect()
    }
    
    fn build_edges(&self) -> Vec<(usize, usize)> {
        self.rooms
            .iter()
            .flat_map(|f| 
                f.tunnels
                    .iter()
                    .map(|t| (self.index_of(&f.name), self.index_of(t))))
            .collect_vec()
    }

    fn build_flow_rates(&self) -> Vec<u32> {
        self.rooms
            .iter()
            .map(|r| r.flow)
            .collect()
    }
    
    fn build_distances_flw(&self) -> Vec<Vec<u32>> {
        let v_dim = self.rooms.len();
        let mut dist = vec![vec![u16::MAX as u32; v_dim]; v_dim];
        
        for (u, v) in &self.edges {
            dist[*u][*v] = 1  // The weight of the edge (u, v)
        }
        for v in 0..v_dim {
            dist[v][v] = 0;
        }
        for k in 0..v_dim {
            for i in 0..v_dim {
                for j in 0..v_dim {
                    if dist[i][j] > dist[i][k] + dist[k][j] {
                        dist[i][j] = dist[i][k] + dist[k][j];
                    }
                }
            }
        }
        
        dist
    }

    fn build_working_valves(&self) -> Vec<usize> {
        self.rooms
            .iter()
            .enumerate()
            .filter(|(_, r)| r.flow > 0)
            .map(|(i, _)| i)
            .collect()
    }
    
    pub fn build_plans(&self, max_time: u32) -> u32 {
        let mut score = Score { max: 0 };
        let mut incomplete = LinkedList::new();
        let valves = self.rooms.iter().filter(|r| r.flow > 0).collect_vec();
        let starting_valve = self.index_of("AA");
        
        let initial_plan = Plan::new(
            valves.iter().map(|r| self.index_of(&r.name)).collect()
        );
        incomplete.push_front(initial_plan);
        
        while let Some(plan) = incomplete.pop_front() {
            if plan.elephant.time >= max_time && plan.player.time >= max_time {
                score.publish_result(&plan);
            } else if plan.closed_valves.len() == 0 {
                score.publish_result(&plan);
            } else {
                for next_valve in &plan.closed_valves {
                    let mut next_plan = plan.clone();
                    next_plan.closed_valves.remove(next_valve);
                    
                    let subplan = next_plan.get_by_time_mut();
                    let flow = self.rooms.get(*next_valve).unwrap().flow;
                    let dist = self.distances[subplan.last_valve(starting_valve)][*next_valve];
                    subplan.add_valve(next_valve, flow, dist, max_time);    
                    
                    incomplete.push_front(next_plan);
                }
            }
        }
        
        score.max
    }

    pub fn build_plans_rec_init(&self, max_time: u32) -> u32
    {
        
        let starting_room = self.index_of("AA");
        let remaining = self.working_valves.iter().cloned().collect();
        self.build_plans_rec(
            max_time,
            max_time, 
            &starting_room,
            &starting_room,
            remaining)
    }
    
    // Gives a bad result though
    pub fn build_plans_rec(&self, pl_time: u32, el_time: u32, pl_room: &usize, el_room: &usize, remaining: HashSet<usize>) -> u32 {
        let mut best = 0u32;
        for next_valve in &remaining {
            let mut new_remaining = remaining.clone();
            new_remaining.remove(&next_valve);
            
            let flow = self.flow_rates[*next_valve];
            
            if pl_time > el_time {
                let dist = self.distances[*pl_room][*next_valve];
                if pl_time > dist + 1 {
                    let new_pl_time = pl_time - dist - 1;
                    let value = new_pl_time * flow;

                    best = best.max(
                        value +
                        self.build_plans_rec(new_pl_time, el_time, next_valve, el_room, new_remaining)
                    )
                } else {
                    continue;
                }
            } else {
                let dist = self.distances[*el_room][*next_valve];
                if el_time > dist + 1 {
                    let new_el_time = el_time - dist - 1;
                    let value = new_el_time * flow;

                    best = best.max(
                        value +
                            self.build_plans_rec(pl_time, new_el_time, pl_room, next_valve, new_remaining)
                    )
                } else {
                    continue;
                }
            }
        }
        best
    }
    
    pub fn build_plans_parallel(&self, max_time: u32) -> u32 {
        let valves = self.rooms.iter().filter(|r| r.flow > 0).collect_vec();
        valves
            .par_iter()
            .map(|r| self.index_of(&r.name))
            .map(|v| self.build_plans_segment(max_time, &valves, v))
            .max()
            .unwrap()
    }
    
    pub fn build_plans_segment(&self, max_time: u32, valves: &Vec<&Room>, segment: usize) -> u32 {
        let mut score = Score { max: 0 };
        let mut incomplete = LinkedList::new();
        let starting_valve = self.index_of("AA");

        // Initialise segment
        let mut initial_plan = Plan::new(
            valves
                .iter()
                .map(|r| self.index_of(&r.name))
                .filter(|i| *i != segment)
                .collect()
        );
        let initial_subplan = initial_plan.get_by_time_mut();
        let flow = self.rooms.get(segment).unwrap().flow;
        let dist = self.distances[initial_subplan.last_valve(starting_valve)][segment];
        initial_subplan.add_valve(&segment, flow, dist, max_time);
        incomplete.push_front(initial_plan);

        // Start loop depth-first
        while let Some(plan) = incomplete.pop_front() {
            if plan.elephant.time >= max_time && plan.player.time >= max_time {
                score.publish_result(&plan);
            } else if plan.closed_valves.len() == 0 {
                score.publish_result(&plan);
            } else {
                for next_valve in &plan.closed_valves {
                    let mut next_plan = plan.clone();
                    next_plan.closed_valves.remove(next_valve);

                    let subplan = next_plan.get_by_time_mut();
                    let flow = self.rooms.get(*next_valve).unwrap().flow;
                    let dist = self.distances[subplan.last_valve(starting_valve)][*next_valve];
                    subplan.add_valve(next_valve, flow, dist, max_time);

                    incomplete.push_front(next_plan);
                }
            }
        }
        
        println!("Finished segment {} with max {}", segment, score.max);
        score.max
    }
}

#[derive(Debug, Clone)]
struct SubPlan {
    valves: Vec<usize>,
    total_flow: u32,
    time: u32
}

impl SubPlan {
    pub fn new() -> SubPlan {
        SubPlan {
            valves: vec![],
            total_flow: 0,
            time: 0,
        }
    }

    pub fn last_valve(&self, default: usize) -> usize {
       self.valves.last().map(|v| *v).unwrap_or(default) 
    }
    
    pub fn add_valve(&mut self, valve: &usize, flow: u32, distance: u32, max_time: u32) {
        self.time += distance + 1;
        if self.time <= max_time {
            self.valves.push(*valve);
            self.total_flow += flow * (max_time - self.time);
        }
    }
}

#[derive(Debug, Clone)]
struct Plan {
    player: SubPlan,
    elephant: SubPlan,
    closed_valves: HashSet<usize>
}

impl Plan {
    pub fn new(closed_valves: HashSet<usize>) -> Plan {
        Plan {
            player: SubPlan::new(),
            elephant: SubPlan::new(),
            closed_valves,
        }
    }
    
    pub fn get_by_time_mut(&mut self) -> &mut SubPlan {
        if self.player.time < self.elephant.time {
            &mut self.player
        }
        else {
            &mut self.elephant
        }
    }
}



pub fn process(input: String) {
    let (_, cave) =  Cave::parse(&input).unwrap();
    
    let now = Instant::now();
    
    let max_score: u32 = 
        cave.build_plans_rec_init(26);
    
    println!("Maximum pressure relieved: {}", max_score);
    println!("Solved in {:.2?}", now.elapsed());
    println!("Correct result: 1933");
}



#[cfg(test)]
mod tests_part16 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parser() {

    }
}