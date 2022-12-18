use std::collections::HashSet;

#[derive(Clone, Debug)]
enum Operation {
    Right,
    Up,
    Left,
    Down
}

impl Operation {
    pub fn parse(input: &str) -> Result<Operation, String> {
        match input {
            "R" => Ok(Operation::Right),
            "U" => Ok(Operation::Up),
            "L" => Ok(Operation::Left),
            "D" => Ok(Operation::Down),
            i => Err(format!("Unable to parse Operation from {}", i))
        }
    }
}

struct Head {
    x: i32,
    y: i32
}

impl Head {
    pub fn new (x: i32, y: i32) -> Head {
        Head { x, y }
    }
    
    pub fn move_to(&mut self, direction: Operation) {
        match direction {
            Operation::Right => self.x += 1,
            Operation::Up => self.y += 1,
            Operation::Left => self.x -= 1,
            Operation::Down => self.y -= 1,
        }
        println!("Moved head {:?} to ({}, {})", direction, self.x, self.y);
    }
}

struct Tail {
    x: i32,
    y: i32,
    previous_positions: Vec<(i32, i32)>
}

impl Tail {

    pub fn new (x: i32, y: i32) -> Tail {
        Tail { x, y, previous_positions: vec![ (x, y) ] }
    }
    
    fn get_tail_position(&self, head: &Head) -> (i32, i32) {
        // This simple approach fails only on no_movement examples because 0.5 rounds up
        // let hpx = ((&head.x + self.x) as f32 / 2.0).round() as i32;
        // let hpy = ((&head.y + self.y) as f32 / 2.0).round() as i32;
        
        // This more complicated approach computes a vector 
        // between head and tail and takes a point of distance 1 from head 
        // in direction of tail 
        
        // In this case the head is over the tail and we would get division by zero below
        if head.x == self.x && head.y == self.y {
            return (head.x, head.y)
        }
        
        // vector v (between head and tail)
        let (vx, vy) = (&head.x - self.x, &head.y - self.y);
        
        // length of vector v
        let vlength = ((vx.pow(2) + vy.pow(2)) as f32).sqrt();
        
        // normalised vector
        let (nx, ny) = ((vx as f32 / vlength).round(), (vy as f32 / vlength).round());
        
        // subtract vector from head, round to a cell
        let (hx, hy) = ((head.x as f32 - nx).round() as i32, (head.y as f32 - ny).round() as i32);
        
        (hx, hy)
    } 
    
    pub fn follow(&mut self, head: &Head) {
        let (x, y) = self.get_tail_position(&head);
        self.previous_positions.push((x, y));
        self.x = x;
        self.y = y;
        println!("Moved tail to ({}; {})", x, y);
    }
}

pub fn process(input: String) {
    let operations: Vec<_>= input
        .lines()
        .flat_map(|l| {
            let words: Vec<_> = l.split(" ").collect();
            let op = Operation::parse(words[0]).unwrap();
            let count = words[1].parse::<usize>().unwrap();
            vec![op; count]
        })
        .collect();

    let mut head = Head { x: 0, y: 0 };
    let mut tail = Tail { x: 0, y: 0, previous_positions: Vec::new() };
    
    for operation in operations {
        head.move_to(operation);
        tail.follow(&head);
    }
    
    let unique_pos: &HashSet<String> = &tail.previous_positions
        .iter()
        .map(|&(x, y)| format!("{};{}", x, y))
        .collect();
    dbg!(&tail.previous_positions);
    println!("Visited {} unique positions", &unique_pos.len());
    println!("6352 is too high!");
}

#[cfg(test)]
mod tests_part9 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_next_tail_position() {
        let tail = Tail::new(0, 0);
        // in line movement
        assert_eq!(tail.get_tail_position(&Head::new(2, 0)), (1, 0));
        assert_eq!(tail.get_tail_position(&Head::new(0, 2)), (0, 1));
        assert_eq!(tail.get_tail_position(&Head::new(-2, 0)), (-1, 0));
        assert_eq!(tail.get_tail_position(&Head::new(0, -2)), (0, -1));
        
        // diagonal movement
        assert_eq!(tail.get_tail_position(&Head::new(1, 2)), (1, 1));
        assert_eq!(tail.get_tail_position(&Head::new(2, 1)), (1, 1));
        assert_eq!(tail.get_tail_position(&Head::new(-1, -2)), (-1, -1));
        assert_eq!(tail.get_tail_position(&Head::new(-2, -1)), (-1, -1));
    
        // no movement
        assert_eq!(tail.get_tail_position(&Head::new(1, 1)), (0, 0));
        assert_eq!(tail.get_tail_position(&Head::new(-1, 1)), (0, 0));
        assert_eq!(tail.get_tail_position(&Head::new(0, 0)), (0, 0));
    }
}
