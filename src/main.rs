#![allow(dead_code)]

use std::{fs, env};
mod part1;
mod part1_2;
mod part2;
mod part2_2;
mod part3;
mod part3_2;
mod part4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename = "part4_sharp.txt".to_string();
    let filename = args.get(1).unwrap_or(&default_filename);
    
    let input = fs::read_to_string(format!("data\\{}", filename))
        .expect("Wrong file location");
    
    part4::process(input);
}
