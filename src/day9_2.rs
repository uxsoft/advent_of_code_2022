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

#[derive(Clone)]
struct Knot {
    x: i32,
    y: i32,
    previous_positions: Vec<(i32, i32)>
}

impl Knot {

    pub fn move_to(&mut self, direction: Operation) {
        match direction {
            Operation::Right => self.x += 1,
            Operation::Up => self.y += 1,
            Operation::Left => self.x -= 1,
            Operation::Down => self.y -= 1,
        }
        //println!("Moved knot {:?} to ({}, {})", direction, self.x, self.y);
    }

    pub fn new (x: i32, y: i32) -> Knot {
        Knot { x, y, previous_positions: vec![ (x, y) ] }
    }
    
    fn get_tail_position(&self, head: &Knot) -> (i32, i32) {
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
    
    pub fn follow(&mut self, head: &Knot) {
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

    let mut knots = vec![Knot::new(0, 0); 10]; 
    
    for operation in operations {
        knots.get_mut(0).unwrap().move_to(operation);
        for i in 1..10 {
            let (head, tail) = knots.split_at_mut(i);
            tail.first_mut().unwrap().follow(head.last().unwrap());
        }
    }
    
    let unique_pos: &HashSet<String> = &knots.last().unwrap().previous_positions
        .iter()
        .map(|&(x, y)| format!("{};{}", x, y))
        .collect();

    println!("Visited {} unique positions", &unique_pos.len());
}

#[cfg(test)]
mod tests_part9 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_next_tail_position() {
        let tail = Knot::new(0, 0);
        // in line movement
        assert_eq!(tail.get_tail_position(&Knot::new(2, 0)), (1, 0));
        assert_eq!(tail.get_tail_position(&Knot::new(0, 2)), (0, 1));
        assert_eq!(tail.get_tail_position(&Knot::new(-2, 0)), (-1, 0));
        assert_eq!(tail.get_tail_position(&Knot::new(0, -2)), (0, -1));
        
        // diagonal movement
        assert_eq!(tail.get_tail_position(&Knot::new(1, 2)), (1, 1));
        assert_eq!(tail.get_tail_position(&Knot::new(2, 1)), (1, 1));
        assert_eq!(tail.get_tail_position(&Knot::new(-1, -2)), (-1, -1));
        assert_eq!(tail.get_tail_position(&Knot::new(-2, -1)), (-1, -1));
    
        // no movement
        assert_eq!(tail.get_tail_position(&Knot::new(1, 1)), (0, 0));
        assert_eq!(tail.get_tail_position(&Knot::new(-1, 1)), (0, 0));
        assert_eq!(tail.get_tail_position(&Knot::new(0, 0)), (0, 0));
    }
}
