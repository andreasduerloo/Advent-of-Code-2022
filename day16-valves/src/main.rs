use std::fs;
use std::env;
use day16_valves::*;
use std::collections::HashMap;

// HashMap <name, Valve>? 

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {
            
            let mut valves: HashMap<&str, &Valve> = HashMap::new();



        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
