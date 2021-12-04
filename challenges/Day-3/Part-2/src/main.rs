use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt");
    let reader = BufReader::new(file.unwrap());
    let bytes_vec: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let oxygen_int: isize = isize::from_str_radix(get_oxygen(bytes_vec.clone()).as_str(), 2).unwrap();
    let co2_int: isize = isize::from_str_radix(get_co2(bytes_vec.clone()).as_str(), 2).unwrap();
    println!("{}", oxygen_int * co2_int);
}

fn get_oxygen(input: Vec<String>) -> String {
    let mut oxygen: Vec<String> = input;
    let mut i = 0;
    while oxygen.len() > 1 {
        let mut zero = 0;
        let mut one = 0;
        for element in oxygen.iter() {
            let character: char = element.chars().nth(i).unwrap();
            if character == '0' {
                zero = zero + 1;
            } else {
                one = one + 1;
            }
        }
        if one >= zero {
            oxygen = remove_elements(oxygen, i, '0');
        } else {
            oxygen = remove_elements(oxygen, i, '1');
        }
        i = i + 1;
    }
    oxygen[0].clone()
}

fn get_co2(input: Vec<String>) -> String {
    let mut co2: Vec<String> = input;
    let mut i = 0;
    while co2.len() > 1 {
        let mut zero = 0;
        let mut one = 0;
        for element in co2.iter() {
            let character: char = element.chars().nth(i).unwrap();
            if character == '0' {
                zero = zero + 1;
            } else {
                one = one + 1;
            }
        }
        if zero <= one {
            co2 = remove_elements(co2, i, '1');
        } else {
            co2 = remove_elements(co2, i, '0');
        }
        i = i + 1;
    }
    co2[0].clone()
}

fn remove_elements(mut input: Vec<String>, index: usize, contains: char) -> Vec<String> {
    input.retain(|elem| elem.chars().nth(index).unwrap() != contains);
    input
}
