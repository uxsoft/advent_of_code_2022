use std::collections::LinkedList;
use regex::Regex;
use itertools::Itertools;

#[derive(Debug)]
enum Operation {
    Multiply(u128),
    Add(u128),
    Square
}

impl Operation {
    pub fn parse(input: &str) -> Operation {
        // * 19
        let words: Vec<_> = input.split(" ").collect();
        match (words[0], words[1]) {
            ("*", "old") => Operation::Square,
            ("*", v) => Operation::Multiply(v.parse().unwrap()),
            (_, v) => Operation::Add(v.parse().unwrap())
        }
    }
    
    pub fn apply(&self, input: u128) -> u128 {
        match self {
            Operation::Multiply(v) => input * v,
            Operation::Add(v) => input + v,
            Operation::Square => input * input
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: u8,
    items: LinkedList<u128>,
    operation: Operation,
    test: u128,
    next_positive: u8,
    next_negative: u8,
    score: u128
}

impl Monkey {
    // Monkey 0:
    // Starting items: 79, 98
    // Operation: new = old * 19
    // Test: divisible by 23
    // If true: throw to monkey 2
    // If false: throw to monkey 3
    
    pub fn parse(input: &str) -> Option<Monkey> {
        let lines: Vec<_> = input.lines().collect();
        let re_id : Regex = Regex::new(r"Monkey (\d+):").unwrap();
        let id = re_id.captures(lines[0].trim())?.get(1)?.as_str().parse().unwrap();

        let re_starting_items : Regex = Regex::new(r"Starting items: ([,\s\d]*)").unwrap();
        let starting_items = re_starting_items
            .captures(lines[1].trim())?
            .get(1)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|i| i.parse::<u128>().unwrap())
            .collect();

        let re_op : Regex = Regex::new(r"Operation: new = old ([\*\+]\s[0-9a-z]+)").unwrap();
        let op = re_op.captures(lines[2].trim())?.get(1)?.as_str();

        let re_test : Regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
        let test = re_test.captures(lines[3].trim())?.get(1)?.as_str().parse().unwrap();

        let re_next_positive : Regex = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
        let next_positive = re_next_positive.captures(lines[4].trim())?.get(1)?.as_str().parse().unwrap();

        let re_next_negative : Regex = Regex::new(r"If false: throw to monkey (\d+)").unwrap();
        let next_negative = re_next_negative.captures(lines[5].trim())?.get(1)?.as_str().parse().unwrap();
                
        Some(Monkey {
            id,
            items: starting_items,
            operation: Operation::parse(op),
            test,
            next_positive,
            next_negative,
            score: 0
        })
    }
    
    pub fn inspect_items(&mut self, modulo: u128) -> Vec<(u8, u128)> {
        let mut outbox = Vec::new();

        while self.items.len() > 0 {
            let item = self.items.pop_front().unwrap();
            self.score += 1;
            
            let v2 = self.operation.apply(item);
            let v3 = v2 % modulo;
            let next_monkey = if v3 % self.test == 0 { self.next_positive } else { self.next_negative };
            
            outbox.push((next_monkey, v3));
        }

        outbox
    }
}

pub fn process(input: String) {
    let mut monkeys : Vec<Monkey> = input
        .split("\n\n")
        .map(Monkey::parse)
        .map(Option::unwrap)
        .collect();
    
    let modulo: u128 = monkeys.iter().map(|m| m.test).product();
    
    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            let outbox = monkeys.get_mut(i).unwrap().inspect_items(modulo);
            for (id, item) in outbox {
                monkeys.get_mut(id as usize).unwrap().items.push_back(item)
            }
        }
    }
    
    dbg!(&monkeys);
    
    let v: &u128 = &monkeys.iter().map(|m| m.score as u128).sorted().rev().take(2).product();
    dbg!(v);
    //println!("Sum of two top monkeys activity is: {}", v);
}