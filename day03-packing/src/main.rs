use std::fs;
use std::env;
use day03_packing::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(content) = fs::read_to_string(filename) {
            
            let mut input_vec: Vec<&str> = content.lines().collect();
            let mut total: u64 = 0;

            while let Some(backpack) = input_vec.pop() {
                total += item_value(find_common(backpack).unwrap());
            }

            println!("⭐ First star ⭐ - Total value is {}.", total);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
