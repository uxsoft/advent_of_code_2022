#![allow(dead_code)]

use std::{fs, env};
mod day1;
mod day1_2;
mod day2;
mod day2_2;
mod day3;
mod day3_2;
mod day4;
mod day4_2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename = "day4_sharp.txt".to_string();
    let filename = args.get(1).unwrap_or(&default_filename);
    
    let input = fs::read_to_string(format!("data\\{}", filename))
        .expect("Wrong file location");
    
    day4_2::process(input);
}
