use std::fs;
use std::env;
use std::collections::HashMap;
use day09_rope::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {
            
            let mut map: Map = HashMap::new();
            let mut map2: Map = HashMap::new();

            let mut head: Point = (0, 0);
            let mut tail: Point = (0, 0);

            let mut rope2 = [(0, 0); 10];

            for i in 0..10 {
                rope2[i] = (0, 0);
            }

            input.lines()
                .map(|s| parse(s))
                .for_each(|inst| {
                    move_head(inst, &mut head, &mut tail, &mut map);
                    move_head2(inst, &mut rope2, &mut map2);
                });

            println!("Points passed: {}", map.len());
            println!("Points passed: {}", map2.len());

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}

