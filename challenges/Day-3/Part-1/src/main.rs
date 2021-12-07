use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut one = 0;
    let mut zero = 0;
    let mut gamma_rate: String = String::new();
    let mut epsilon_rate: String = String::new();
    let file = File::open("input.txt");
    let reader = BufReader::new(file.unwrap());
    let bytes_vec: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    for i in 0..12 {
        for element in bytes_vec.iter() {
            let character: char = element.chars().nth(i).unwrap();
            if character == '1' {
                one += 1;
            } else if character == '0' {
                zero += 1;
            }
        }
        if one > zero {
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0");
        } else {
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
        }
        one = 0;
        zero = 0;
    }
    let gamma_rate_int: isize = isize::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate_int: isize = isize::from_str_radix(epsilon_rate.as_str(), 2).unwrap();
    println!("{}", gamma_rate_int * epsilon_rate_int);
}
