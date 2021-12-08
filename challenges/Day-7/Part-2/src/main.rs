use std::fs;

fn main() {
    let mut horizontal_positions: Vec<u16> = fs::read_to_string("input.txt")
        .expect("Unable to open file")
        .split(",")
        .map(|number| number.parse::<u16>().expect("Unable to convert to u16"))
        .collect();
    horizontal_positions.sort();
    let mut fuel_costs: Vec<u32> = Vec::new();
    for i in 0..=horizontal_positions.len() - 1 {
        let mut steps: u32 = 0;
        let mut fuel_cost: u32 = 0;
        for position in horizontal_positions.iter() {
            let steps = *position as i16 - i as i16;
            for i in 1..=steps.abs() as u32 {
                fuel_cost = fuel_cost + i;
            }
        }
        fuel_costs.push(fuel_cost);
    }
    println!("Smallest fuel cost: {:?}", smallest_fuel_cost(&fuel_costs));
}

fn smallest_fuel_cost(vec: &Vec<u32>) -> u32 {
    let mut smallest_cost: u32 = std::u32::MAX;
    for number in vec {
        if number < &(smallest_cost as u32) {
            smallest_cost = *number as u32;
        }
    }
    smallest_cost
}
