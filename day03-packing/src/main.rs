use std::fs;
use std::env;
use day03_packing::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(content) = fs::read_to_string(filename) {
            
            let mut input_vec: Vec<&str> = content.lines().collect();
            let mut totals: [u64; 2] = [0, 0];

            // Start with the second star, we'll consume the vector with an iterator for the first one

            for i in 0..100 {
                let three_backpacks: Vec<&str> = Vec::from([input_vec[i * 3], input_vec[(i * 3) + 1], input_vec[(i * 3) + 2]]);

                totals[1] += item_value(find_badge(three_backpacks).unwrap());
            }

            // First star

            while let Some(backpack) = input_vec.pop() {
                totals[0] += item_value(find_common(backpack).unwrap());
            }

            println!("⭐ First star ⭐ - Total value is {}.", totals[0]);
            println!("🌟 Second star ✨ - Total value of the badges is {}.", totals[1]);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
