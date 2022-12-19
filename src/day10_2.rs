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
    
    for i in 0..240 { // CRT cycles
        let x = i % 40;
        if x == 0 {
            println!()
        }
        let sprite_loc = cpu.history_x.get(i).unwrap();
        
        if (x as i32 >= sprite_loc - 1) && (x as i32 <= sprite_loc + 1) {
            print!("#")
        } else {
            print!(".");
        }
    }
}

#[cfg(test)]
mod tests_part10 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_next_tail_position() {

    }
}
