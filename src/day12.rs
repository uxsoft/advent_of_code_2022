use std::collections::{HashMap, LinkedList};

#[derive(Debug)]
struct Field {
    x: usize,
    y: usize,
    altitude: u8,
    is_start: bool, 
    is_exit: bool,
}

impl Field {
    pub fn new(x: usize, y: usize) -> Field {
        Field {
            x,
            y,
            altitude: 0,
            is_start: false,
            is_exit: false,
        }
    }
    
    pub fn parse(input: char, x: usize, y: usize) -> Field {
        match input {
            'S' => Field {
                altitude: 0,
                is_start: true,
                .. Field::new(x, y)
            },
            'E' => Field {
                altitude: 25,
                is_exit: true,
                .. Field::new(x, y)
            },
            n => Field {
                altitude: n as u8 - 97,
                .. Field::new(x, y)
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    fields: Vec<Vec<Field>>
}

impl Map {
    pub fn parse(input: &str) -> Map {
        let fields: Vec<Vec<_>> = input
            .lines()
            .enumerate()
            .map(|(y, l)| l.chars()
                .enumerate()
                .map(|(x, ch)| Field::parse(ch, x, y))
                .collect())
            .collect();
        
        Map {
            height: fields.len(),
            width: fields.first().unwrap().len(),
            fields
        }
    }
    
    pub fn get(&self, x: usize, y: usize) -> &Field {
        self.fields.get(y).unwrap().get(x).unwrap()
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Field {
        self.fields.get_mut(y).unwrap().get_mut(x).unwrap()
    }
    
    pub fn as_mut(&mut self, item: &Field) -> &mut Field {
        self.get_mut(item.x, item.y)
    }
    
    pub fn successors(&self, x: usize, y: usize) -> Vec<&Field> {
        let this = self.get(x, y);
        
        vec![(x as i32 - 1, y as i32), (x as i32 + 1, y as i32), (x as i32, y as i32 - 1), (x as i32, y as i32 + 1)]
            .iter()
            .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < self.width as i32 && *y < self.height as i32)
            .map(|(x, y)| self.get(*x as usize, *y as usize))
            .filter(|i| i.altitude <= this.altitude + 1)
            .collect()
    }
    
    pub fn find_start(&self) -> &Field {
        self.fields.iter().flatten().find(|i| i.is_start).unwrap()
    }
    
    pub fn find_exit(&self) -> &Field {
        self.fields.iter().flatten().find(|i| i.is_exit).unwrap()
    }
    
    // Assume that all cells are initialised to u32::MAX except start which is initialised to 0
    pub fn distance_to(&self, start: &Field, end: &Field) -> u32 {
        let mut queue = LinkedList::new();
        let mut distances: HashMap<(usize, usize), u32> = HashMap::new();
        
        distances.insert((start.x, start.y), 0);
        queue.push_back(start);
        
        while queue.len() > 0 {
            let item = queue.pop_front().unwrap();
            let distance_from_item_to_next = distances.get(&(item.x, item.y)).unwrap() + 1;

            for next in self.successors(item.x, item.y) {
                let next_distance = *distances.get(&(next.x, next.y)).unwrap_or(&u32::MAX);
                
                if next_distance > distance_from_item_to_next {
                    // We have a better way to this cell, need to update it.
                    distances.insert((next.x, next.y), distance_from_item_to_next);
                    
                    // Since we have a better way for this item, we should re-calculate all the paths from it as well.
                    queue.push_back(next);
                }
            }
        }
        
        *distances.get(&(end.x, end.y)).unwrap_or(&u32::MAX)
    }
}

pub fn process(input: String) {
    let mut map = Map::parse(&input);
    let exit = map.find_exit();
    
    let min_distance = map.fields
        .iter()
        .flatten()
        .filter(|i| i.altitude == 0)
        .map(|i| map.distance_to(i, exit))
        .min()
        .unwrap();
    
    println!("Minimal distance: {}", min_distance)
}
