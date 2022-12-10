use std::fs;
use std::env;
use std::collections::HashMap;
use day09_rope::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {
            
            let mut map1: Map = HashMap::new();
            let mut map2: Map = HashMap::new();

            let mut rope1 = [(0, 0); 2];
            let mut rope2 = [(0, 0); 10];

            input.lines()
                .map(|s| parse(s))
                .for_each(|inst| {
                    move_rope(inst, &mut rope1, &mut map1);
                    move_rope(inst, &mut rope2, &mut map2);
                });

            println!("⭐ First star ⭐ - The tail has passed {} points.", map1.len());
            println!("🌟 Second star ✨ - The tail has passed {} points.", map2.len());

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}

