use std::cmp::{min, max};
use nom::{IResult, multi::separated_list1, sequence::separated_pair, bytes::complete::tag, character::complete::newline};

// 498,4 -> 498,6 -> 496,6
pub fn parse_rock(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(
        tag(" -> "), 
        separated_pair(nom::character::complete::u32, tag(","), nom::character::complete::u32)
    )(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Vec<(u32, u32)>>> {
    separated_list1(newline, parse_rock)(input)
}

#[derive(Debug, Clone)]
enum Cell {
    Air,
    Rock,
    Sand
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            Cell::Air => ".".to_owned(),
            Cell::Rock => "#".to_owned(),
            Cell::Sand => "o".to_owned(),
        }
    }
}

enum Stability {
    Stable,
    Fall(u32, u32),
    OutOfBounds
}

#[derive(Debug)]
struct Grid {
    offset_x: u32,
    cells: Vec<Vec<Cell>>
}

impl Grid {
    pub fn new(w: usize, h: usize, offset_x: u32) -> Grid {
        Grid {
            offset_x,
            cells: vec![vec![Cell::Air; w]; h]
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&Cell> {
        self.cells
            .get(y as usize)
            .and_then(|r| r.get(x as usize - self.offset_x as usize))
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Cell> {
        self.cells
            .get_mut(y as usize)
            .and_then(|r| r.get_mut(x as usize - self.offset_x as usize))
    }

    pub fn draw_rock_line(&mut self, from: &(u32, u32), to: &(u32, u32)) {
        for x in min(from.0, to.0)..=max(from.0, to.0) {
            for y in min(from.1, to.1)..=max(from.1, to.1) {
                let ptr = self.get_mut(x, y).unwrap();
                *ptr = Cell::Rock;
            }
        }
    }

    pub fn draw_rock_chain(&mut self, chain: &Vec<(u32, u32)>) {
        for i in chain.windows(2) {
            self.draw_rock_line(i.first().unwrap(), i.last().unwrap());
        }
    }
    
    pub fn draw_sand(&mut self, x: u32, y: u32) {
        let ptr = self.get_mut(x, y).unwrap();
        *ptr = Cell::Sand;
    }
    
    fn sand_next_pos(&self, x: u32, y: u32) -> Stability {
        match self.get(x, y + 1) {
            Some(Cell::Air) => Stability::Fall(x, y + 1),
            Some(Cell::Rock) | Some(Cell::Sand) => {
                if let Some(&Cell::Air) = self.get(x - 1, y + 1) { 
                    Stability::Fall(x - 1, y + 1)
                } else if let Some(&Cell::Air) = self.get(x + 1, y + 1) {
                    Stability::Fall(x + 1, y + 1)
                } else { Stability::Stable }
            },
            None => Stability::OutOfBounds
        }
    }
    
    pub fn spawn_sand(&mut self, x0: u32, y0: u32) -> bool
    {
        let mut current_loc = (x0, y0);
        
        while let Some(&Cell::Air) = self.get(x0, y0) {
            match self.sand_next_pos(current_loc.0, current_loc.1) {
                Stability::Stable => {
                    self.draw_sand(current_loc.0, current_loc.1);
                    return true
                },
                Stability::Fall(x, y) => {
                    current_loc = (x, y)
                }
                Stability::OutOfBounds => {
                    return false
                }
            }
        }
        false
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        self.cells
            .iter()
            .map(|l| 
                l.iter().map(ToString::to_string).collect::<Vec<_>>().join(""))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub fn process(input: String) { 
    let (_, rocks) = parse_input(&input).unwrap();
    let mut grid = Grid::new(1000, 200, 0);
    
    for chain in &rocks {
        grid.draw_rock_chain(&chain);
    }
    
    let max_y = &rocks.iter().flatten().map(|(_, y)| y).max().unwrap();
    grid.draw_rock_line(&(0, **max_y + 2), &(999, **max_y + 2));
    
    let mut i = 0;
    while grid.spawn_sand(500, 0) { 
        i += 1;
        //println!("{}", grid.to_string());
    }
    
    println!("{}", grid.to_string());
    println!("{} pieces of sand fit in", i);
}