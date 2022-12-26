use std::fs;
use std::env;
use day16_valves::*;
use std::collections::HashMap;

// HashMap <name, Valve>? 

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {
            
            let mut flowrates: HashMap<&str, usize> = HashMap::new();
            let mut neighbors: HashMap<&str, Vec<&str>> = HashMap::new();

            for line in input.lines() {
                read_valve(line, &mut flowrates, &mut neighbors);
            }

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
