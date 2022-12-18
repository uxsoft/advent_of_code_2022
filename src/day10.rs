enum Operation {
    Noop,
    Addx(i32)
}

impl Operation {
    fn parse(input: &str) -> Operation {
        let words: Vec<_> = input.split(" ").collect();
            
        match words[0] {
            "addx" => Operation::Addx(words[1].parse::<i32>().unwrap()),
            _ => Operation::Noop
        }
    }
}

struct Cpu {
    reg_x: i32,
    history_x: Vec<i32>
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            reg_x: 1,
            history_x: Vec::new()
        }
    }
    
    fn step(&mut self, op: Operation) {
        match op {
            Operation::Noop => {
                self.history_x.push(self.reg_x);
            },
            Operation::Addx(v) => {
                self.history_x.push(self.reg_x);
                self.history_x.push(self.reg_x);
                self.reg_x += v;
            }
        }
    }
    
}

pub fn process(input: String) {
    let ops: Vec<_> = input.lines().map(Operation::parse).collect();
    
    let mut cpu = Cpu::new();
    for op in ops {
        cpu.step(op);
    }
    
    let mut i = 20; // index of cycle 20 is 19
    let mut sum = 0;
    while i < cpu.history_x.len() {
        println!("Cycle {} value {}", i, cpu.history_x[i - 1]);
        
        sum += cpu.history_x[i - 1] * (i as i32);
        i += 40;
    }
    
    dbg!(cpu.history_x);
    println!("Product is {}", sum);
}

#[cfg(test)]
mod tests_part10 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_next_tail_position() {

    }
}
