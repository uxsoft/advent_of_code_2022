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
  
    
    let y = 2000000;
    let mut counter = 0;
    for x in -10_000_000..10_000_000 {
        let i = Spot { x, y };
        
        if sensors.iter().any(|s| s.in_range(&i)) {
            if !sensors.iter().any(|s| s.beacon.distance(&i) == 0) {
                counter += 1;
            }
        }
    }
    
    println!("Found {} spots where the distress beacon can't be", counter);
    
    //dbg!(sensors);
}