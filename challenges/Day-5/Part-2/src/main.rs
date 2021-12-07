use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let mut coordinates_hash = HashMap::new();
    for line in data.lines() {
        let line_coordinates = line.split(" -> ").collect::<Vec<&str>>();
        let parsed_coordinates = line_coordinates
            .iter()
            .map(|coordinate| {
                coordinate
                    .split(",")
                    .map(|x| x.parse::<u16>().unwrap())
                    .collect::<Vec<u16>>()
            })
            .collect::<Vec<Vec<u16>>>();
        let derivative: i16 = (parsed_coordinates[1][1] as i16 - parsed_coordinates[0][1] as i16)
            .checked_div(parsed_coordinates[1][0] as i16 - parsed_coordinates[0][0] as i16)
            .unwrap_or_else(|| 0);
        if derivative == 1 || derivative == -1 {
            let mut start = parsed_coordinates[0][0];
            let mut end = parsed_coordinates[1][0];
            if start > end {
                start = parsed_coordinates[1][0];
                end = parsed_coordinates[0][0];
            }
            for i in start..=end {
                let y = derivative * (i as i16 - parsed_coordinates[0][0] as i16)
                    + parsed_coordinates[0][1] as i16;
                *coordinates_hash.entry((i, y as u16)).or_insert(0) += 1;
            }
        } else if parsed_coordinates[0][0] == parsed_coordinates[1][0] {
            let mut start: u16 = parsed_coordinates[0][1];
            let mut end: u16 = parsed_coordinates[1][1];
            if end < start {
                start = parsed_coordinates[1][1];
                end = parsed_coordinates[0][1];
            }
            for y in start..=end {
                *coordinates_hash
                    .entry((parsed_coordinates[0][0], y))
                    .or_insert(0) += 1;
            }
        } else if parsed_coordinates[0][1] == parsed_coordinates[1][1] {
            let mut start: u16 = parsed_coordinates[0][0];
            let mut end: u16 = parsed_coordinates[1][0];
            if end < start {
                start = parsed_coordinates[1][0];
                end = parsed_coordinates[0][0];
            }
            for x in start..=end {
                *coordinates_hash
                    .entry((x, parsed_coordinates[0][1]))
                    .or_insert(0) += 1;
            }
        }
    }
    let mut amount: u16 = 0;
    for val in coordinates_hash.values() {
        if *val > 1 {
            amount += 1;
        }
    }
    println!("{}", amount);
}
