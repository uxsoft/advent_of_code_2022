use std::collections::{HashMap, LinkedList};
use regex::Regex;

#[derive(Debug)]
pub struct Crane {
    stacks: HashMap<u8, LinkedList<char>>,
}

impl Crane {
    pub fn new(n: u8) -> Crane {
        let mut stacks = HashMap::new();
        for i in 0..n {
            stacks.insert(i, LinkedList::new());
        }

        Crane { stacks }
    }

    pub fn parse(input: &str) -> Crane {
        let lines: Vec<&str> = input.lines().collect();
        let stack_count = (lines.first().unwrap().len() + 1) / 4;
        let mut crane = Crane::new(stack_count.try_into().unwrap());

        for l in 0..(lines.len() - 1) { //Ignoring last line with indexes; CARE they start from 1 not 0
            let line = lines[l];

            for i in 0..stack_count {
                let crate_content = line.chars().nth(1 + i * 4).unwrap();
                
                if crate_content != ' ' {
                    let stack = crane.stacks.get_mut(&i.try_into().unwrap()).unwrap();
                    stack.push_back(crate_content);
                }
            }
        }
        crane
    }
    
    pub fn move_crates(&mut self, n: u8, source: u8, target: u8) {
        let source_stack = self.stacks.get_mut(&source).unwrap();
        
        let mut picked_up = LinkedList::new(); //This will be in the opposite order so we can take from the front

        for _ in 0..n {
            let elf_crate = source_stack.pop_front().expect("Error: attempted to lift an elf crate from an empty stack");
            picked_up.push_front(elf_crate);
        }

        let target_stack = self.stacks.get_mut(&target).unwrap();

        for _ in 0..n {
            let elf_crate = picked_up.pop_front().expect("Error: attempted to lift an elf crate from an empty stack");
            target_stack.push_front(elf_crate);
        }
    }

    pub fn topmost_crates(&self) -> String {
        let mut s = String::new();

        for i in 0..self.stacks.len() {
            let topmost = self.stacks.get(&i.try_into().unwrap()).unwrap().front().unwrap();
            s.push(*topmost);
        }

        s
    }
}

pub fn process(input: String) {
    let input_segments = input.split("\r\n\r\n").collect::<Vec<_>>();
    let steps_str = input_segments[1].lines();

    let mut crane = Crane::parse(input_segments[0]);
    println!("Crane:");
    dbg!(&crane);

    let re : Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for step_str in steps_str {
        let cap = re.captures(step_str).unwrap();
        crane.move_crates(cap[1].parse::<u8>().unwrap(), cap[2].parse::<u8>().unwrap() - 1, cap[3].parse::<u8>().unwrap() - 1);
    }

    dbg!(&crane);
    println!("{}", &crane.topmost_crates());
}

#[cfg(test)]
mod tests_part5 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_overlapping_with() {}
}
