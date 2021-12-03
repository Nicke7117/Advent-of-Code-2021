use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt");
    let reader = BufReader::new(file.unwrap());
    let mut horizontal_pos: u16 = 0;
    let mut depth: u16 = 0;
    for line in reader.lines() {
        let instruction: Vec<String> = line
            .unwrap()
            .trim()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect();
        println!("{:?}", instruction);
        match instruction[0].as_str() {
            "forward" => horizontal_pos = horizontal_pos + instruction[1].parse::<u16>().unwrap(),
            "up" => depth = depth - instruction[1].parse::<u16>().unwrap(),
            "down" => depth = depth + instruction[1].parse::<u16>().unwrap(),
            _ => println!("Ain't special"),
        }
    }
    println!("{}", horizontal_pos);
    println!("{}", depth);
}
