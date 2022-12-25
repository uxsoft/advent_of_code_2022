use std::collections::{HashMap, LinkedList};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag};
use nom::character::complete::{alpha1, newline};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use rayon::prelude::*;

#[derive(Debug, Clone)]
enum Action {
    Move { from: String, to: String },
    OpenValve { room_name: String, flow: u32 }
}

#[derive(Debug, PartialEq)]
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
    
    pub fn actions(&self, plan: &Plan) -> Vec<Action> {
        let mut actions = Vec::new();

        if self.flow > 0 && plan.is_valve_open(&self.name) {
            actions.push(Action::OpenValve { room_name: self.name.clone(), flow: self.flow });
        }

        for tunnel in &self.tunnels {
            actions.push(Action::Move { from: self.name.clone(), to: tunnel.clone() });
        }
        actions
    }
    
}

#[derive(Debug, Clone)]
struct Plan {
    actions: LinkedList<Action>
}

impl Plan { 
    pub fn new () -> Self {
        Self {
            actions: LinkedList::new(),
        }
    }
    
    pub fn is_valve_open(&self, valve: &str) -> bool {
        for action in &self.actions {
            match action {
                Action::Move { .. } => {}
                Action::OpenValve { room_name, .. } => {
                    if room_name == valve {
                        return true;
                    }
                }
            }
        }
        false
    }
    
    pub fn last_room(&self) -> &str {
        for action in &self.actions {
            match action {
                Action::Move { to, .. } => return to,
                Action::OpenValve { .. } => {}
            }
        }
        "AA"
    }
    
    pub fn score(&self) -> u32 { 
        self.actions
            .iter()
            .enumerate()
            .map(|(i, a)| {
                match a {
                    Action::Move { .. } => 0,
                    Action::OpenValve { flow, .. } => *flow * (29 - i as u32)
                }
            })
            .sum()
    }
    
    pub fn is_full(&self) -> bool {
        self.actions.len() >= 30
    }
}

#[derive(Debug)]
struct Cave {
    rooms: Vec<Room>,
    edges: Vec<(usize, usize)>,
    distances: Vec<Vec<u32>>,
    names: HashMap<String, usize>
}

impl Cave {
    pub fn new(rooms: Vec<Room>) -> Self {
        let mut cave = Cave {
            rooms,
            edges: vec![],
            distances: vec![],
            names: HashMap::new(),
        };
        
        cave.names = cave.build_names();
        cave.edges = cave.build_edges();
        cave.distances = cave.distances_flw();
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
    
    pub fn build_names(&self) -> HashMap<String, usize> {
        self.rooms
            .iter()
            .enumerate()
            .map(|(i, r)| (r.name.clone(), i))
            .collect()
    }
    
    pub fn build_edges(&self) -> Vec<(usize, usize)> {
        self.rooms
            .iter()
            .flat_map(|f| 
                f.tunnels
                    .iter()
                    .map(|t| (self.index_of(&f.name), self.index_of(t))))
            .collect_vec()
    }
    
    pub fn distances_flw(&self) -> Vec<Vec<u32>> {
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
    
    /// Assuming we have a graph G<V, E>, this solution would have a complexity E(V)^30. 
    /// Assuming average of E(V) of ±2 this would result in 1073741824 cycles
    pub fn build_plans(&self) -> Vec<Plan> {
        let mut complete = Vec::new();
        let mut incomplete = LinkedList::new();
        incomplete.push_front(Plan::new());
        
        while let Some(plan) = incomplete.pop_front() {
            if plan.is_full() {
                complete.push(plan);
            }
            else {
                let room = self.get_by_name(plan.last_room());
                for action in room.actions(&plan) {
                    let mut new_plan = plan.clone();
                    new_plan.actions.push_back(action);
                    incomplete.push_back(new_plan);
                }
            }
        }
        
        complete
    }
    
    /// This approach assumes that we care about only valves so it permutes the non-zero valves and then constructs paths between them
    /// Assuming graph G<V,E> this solution has complexity V! (factorial of V)
    /// Assuming average V of 10 this would result in 3628800
    pub fn build_plans2(&self) -> u32 {
        let valves = self.rooms.iter().filter(|r| r.flow > 0).collect_vec();
        let perms = valves
            .iter()
            .permutations(7)
            .collect_vec();
        
        println!("Ranking {} permutations", perms.len());
        let max_score = perms
            .par_iter()
            .map(|permutation| {
                let mut i = 0; 
                let mut total_flow = 0;
                let mut prev_valve = self.index_of("AA");
                for valve in permutation {
                    let valve_index = self.index_of(&valve.name);
                    let dist = self.distances[prev_valve][valve_index];
                    i += dist + 1;
                    prev_valve = valve_index;
                    if i < 30 {
                        total_flow += valve.flow * (30 - i)
                    }
                    else {
                        return total_flow
                    }
                }
                total_flow
            })
            .max()
            .unwrap();
        
        max_score
    }
}

pub fn process(input: String) {
    let (_, cave) =  Cave::parse(&input).unwrap();
    
    let max_score: u32 = 
        cave.build_plans2();
    
    println!("Maximum pressure relieved: {}", max_score);
    println!("1660 is too high")
}



#[cfg(test)]
mod tests_part16 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parser() {

    }
}