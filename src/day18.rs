use std::collections::LinkedList;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};

trait ApproxCmp {
    fn approx_eq(&self, other: f32) -> bool;
}

impl ApproxCmp for f32 {
    fn approx_eq(&self, other: f32) -> bool {
        (self - other).abs() <= self.abs().max(other.abs()) * f32::EPSILON
    }
}

struct Droplet {
    x: usize,
    y: usize,
    z: usize,
}

impl Droplet {
    pub fn distance(&self, other: &Droplet) -> f32 {
        ((self.x.abs_diff(other.x).pow(2) +
            self.y.abs_diff(other.y).pow(2) +
            self.z.abs_diff(other.z).pow(2)) as f32).sqrt()
    }
    pub fn is_touching(&self, other: &Droplet) -> bool {
        //self.distance(other).approx_eq(1.0)
        (self.x.abs_diff(other.x) +
            self.y.abs_diff(other.y) +
            self.z.abs_diff(other.z)) == 1
    }
    pub fn parse(input: &str) -> IResult<&str, Droplet> {
        map(tuple((
            nom::character::complete::u32,
            preceded(tag(","), nom::character::complete::u32),
            preceded(tag(","), nom::character::complete::u32)
        )), |(x, y, z)| Droplet { x: x as usize, y: y as usize, z: z as usize })(input)
    }
}

struct Space {
    items: Vec<Droplet>,

}

fn parse_points(input: &str) -> IResult<&str, Vec<Droplet>> {
    separated_list1(newline, Droplet::parse)(input)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Visited,
    Unvisited,
    Rock,
}

fn neighbors(x: usize, y: usize, z: usize) -> Vec<(usize, usize, usize)> {
    let mut q: Vec<(usize, usize, usize)> = vec![];
    if x > 0 {
        q.push((x - 1, y, z));
    }
    if x < 24 {
        q.push((x + 1, y, z));
    }
    if y > 0 {
        q.push((x, y - 1, z));
    }
    if y < 24 {
        q.push((x, y + 1, z));
    }
    if z > 0 {
        q.push((x, y, z - 1));
    }
    if z < 24 {
        q.push((x, y, z + 1));
    }
    q
}

fn flood_fill(grid: &mut [[[State; 25]; 25]; 25]) -> u32 {
    let mut hit_rock = 0u32;
    let mut q = LinkedList::new();
    q.push_back((0usize, 0usize, 0usize));

    while let Some((x, y, z)) = q.pop_front() {
        
        // match grid[x][y][z] {
        //     State::Visited => continue,
        //     State::Unvisited => grid[x][y][z] = State::Visited,
        //     State::Rock => {
        //         hit_rock += 1;
        //         println!("({}, {}, {}): {}", x, y, z, &hit_rock);
        //     }
        // }
        for (nx, ny, nz) in neighbors(x, y, z) {
            match grid[nx][ny][nz] {
                State::Unvisited => {
                    q.push_back((nx, ny, nz));
                    grid[nx][ny][nz] = State::Visited;
                }
                State::Visited => {}
                State::Rock => {
                    hit_rock += 1;
                    println!("[{}] Hit rock ({}, {}, {}) from ({}, {}, {})", &hit_rock, nx, ny, nz, x, y, z);
                }
            }
        }
    }

    hit_rock
}


fn part2(input: String) -> usize {
    let (_, cubes) = parse_points(&input).unwrap();

    // let max_x = cubes.iter().map(|i| i.x).max().unwrap();
    // let max_y = cubes.iter().map(|i| i.y).max().unwrap();
    // let max_z = cubes.iter().map(|i| i.z).max().unwrap();

    let mut grid = [[[State::Unvisited; 25]; 25]; 25];
    for cube in cubes {
        // Move the droplet 1 to the right, so we can flood fill from all sides
        grid[cube.x + 1][cube.y + 1][cube.z + 1] = State::Rock; 
    }

    let sides = flood_fill(&mut grid);
    sides as usize
}

fn part1(input: String) -> usize {
    let (_, cubes) = parse_points(&input).unwrap();

    let cube_sides = cubes.len() * 6;
    let cube_touches =
        cubes
            .iter()
            .combinations(2)
            .filter(|l| l[0].is_touching(l[1]))
            .count();

    cube_sides - (cube_touches * 2) // because a.b and b.a
}

pub fn process(input: String) {
    let result = part2(input);
    println!("Surface area: {}", result);
    // 2602 is too low
}


#[cfg(test)]
mod tests_part18 {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_touching() {
        let a = Droplet { x: 1, y: 1, z: 1 };
        let b = Droplet { x: 2, y: 1, z: 1 };
        let c = Droplet { x: 4, y: 1, z: 1 };
        assert_eq!(a.is_touching(&b), true);
        assert_eq!(a.is_touching(&c), false);
        assert_eq!(a.is_touching(&a), false);
    }

    #[test]
    fn test_example1() {
        let input = include_str!("../data/day18_dry.txt");
        let result = part1(input.to_owned());
        assert_eq!(result, 64);
    }

    #[test]
    fn test_example2() {
        let input = include_str!("../data/day18_dry.txt");
        let result = part2(input.to_owned());
        assert_eq!(result, 58);
    }
}