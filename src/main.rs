#![allow(dead_code)]

use std::{fs, env};
use std::path::Path;

mod day1;
mod day1_2;
mod day2;
mod day2_2;
mod day3;
mod day3_2;
mod day4;
mod day4_2;
mod day5;
mod day5_2;
mod day6;
mod day7;
mod day7_2;
mod day8;
mod day8_2;
mod day9;
mod day9_2;
mod day10;
mod day10_2;
mod day11;
mod day11_2;
mod day12;
mod day13;
mod day14;
mod day14_2;
mod day15;
mod day15_2;
mod day16;
mod day16_2;
mod day17;
mod day18;
mod day25;
mod day19;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename = "day19_sharp.txt".to_string();
    let filename = args.get(1).unwrap_or(&default_filename);
    
    let input = fs::read_to_string(Path::new("data").join(filename))
        .expect("Wrong file location");
    
    day19::process(input);
}
