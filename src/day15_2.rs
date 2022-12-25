use itertools::Itertools;
use rayon::prelude::*;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

#[derive(Debug)]
pub struct Spot {
    x: i32,
    y: i32,
}

impl Spot {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    pub fn distance(&self, other: &Spot) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
pub struct Sensor {
    sensor: Spot,
    beacon: Spot,
    range: u32 
}

impl Sensor {
    pub fn new(sx: i32, sy: i32, bx: i32, by: i32) -> Self {
        let sensor = Spot { x: sx, y: sy };
        let beacon = Spot { x: bx, y: by };
        let range = sensor.distance(&beacon);
        
        Sensor {
            sensor,
            beacon,
            range
        }
    }
    
    pub fn in_range(&self, other: &Spot) -> bool {
        self.sensor.distance(other) <= self.range
    }
    
    pub fn range_at(&self, y: i32) -> Option<(i32, i32)> {
        let range = self.range as i32 - self.sensor.y.abs_diff(y) as i32;
        if range > 0 {
            Some((self.sensor.x - range, self.sensor.x + range))
        }
        else {
            None
        }
    }
}

// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
pub fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    map(tuple((
        tag("Sensor at x="),
        nom::character::complete::i32, 
        tag(", y="), 
        nom::character::complete::i32,
        tag(": closest beacon is at x="),
        nom::character::complete::i32,
        tag(", y="),
        nom::character::complete::i32)
    ), |(_, b, _, d, _, f, _, h)| Sensor::new(b, d, f, h))(input)
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(newline, parse_sensor)(input)
}

pub fn process(input: String) {
    let (_, sensors) = parse_input(&input).unwrap();    
    
    let _cr: Vec<_> = sensors.iter().filter(|s| s.in_range(&Spot{ x: 14, y: 11 })).collect();
    
    // let loc = (100_000..200_000) // 4_000_000
    //     .into_par_iter()
    //     .flat_map(|y| (0..4_000_000).map(|x| (x, y)).collect_vec())
    //     .find_any(|(x, y)| {
    //         let i = Spot::new(*x, *y);
    //         return !sensors.iter().any(|s| s.in_range(&i))
    //     })
    //     .unwrap_or((-1, -1));
    
    let loc = (0..4_000_000)
        .into_par_iter()
        .flat_map(|y| {
            sensors
                .iter()
                .map(|s| s.range_at(y))
                .filter(|o: &Option<_>| o.is_some())
                .flat_map(|o| vec![(o.unwrap().0 - 1, y), (o.unwrap().1 + 1, y)])
                .collect_vec()
        })
        .filter(|(x, _y)| *x > 0 && *x < 4_000_000)
        .find_any(|(x, y)| {
            let i = Spot::new(*x, *y);
            return !sensors.iter().any(|s| s.in_range(&i))
        })
        .unwrap_or((-1, -1));
    
        
    let score: u128 = loc.0 as u128 * 4000000 + loc.1 as u128;
    
    println!("Found possible distress beacon location at ({}, {}) with score {}", loc.0, loc.1, score);
    
    //dbg!(sensors);
}