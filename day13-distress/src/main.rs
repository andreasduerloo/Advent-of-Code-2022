use std::fs;
use std::env;
use day13_distress::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {
            
            let solution: usize = input.split("\r\n\r\n")
                .enumerate()
                .map(|(i, val)| (i, is_ordered(val)))
                .filter(|tup| tup.1)
                .map(|tup| tup.0)
                //.reduce(|acc, val| acc + val)
                //.unwrap();
                .sum();

            println!("First star - solution: {}", solution);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
