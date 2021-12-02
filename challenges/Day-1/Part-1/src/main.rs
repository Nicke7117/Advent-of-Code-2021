use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input.txt").unwrap();
    let mut larger_than_previous: u16 = 0;
    let mut previous_line: Option<u16> = None;
    for line in file.lines(){
        let line: u16 = line.parse::<u16>().unwrap();
        if let Some(value) = previous_line {
            if line > value {
                larger_than_previous = larger_than_previous + 1;
            }
        }
        previous_line = Some(line);
    } 
    println!("{}", larger_than_previous)
}
