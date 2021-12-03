use std::io::BufReader; 
use std::io::BufRead; 
use std::fs; 

fn main(){
    let file_in = fs::File::open("input.txt").unwrap(); 
    let file_reader = BufReader::new(file_in); 
    let vec: Vec<u16> = file_reader.lines().map(|x| x.unwrap().parse::<u16>().unwrap()).collect();
    println!("{:?}", vec);
    let mut previous_sum: Option<u16> = None;
    let mut larger_than_previous: u16 = 0;
    for i in 1..vec.len() - 1{
        println!("{}", i);
        println!("{}", vec[i + 1]);
        let sum: u16 = vec[i - 1] + vec[i] + vec[i + 1];
        if let Some(value) = previous_sum{
            if value < sum{
                larger_than_previous = larger_than_previous + 1;
            }
        }
        previous_sum = Some(sum);
    }
    println!("{}", larger_than_previous);
}