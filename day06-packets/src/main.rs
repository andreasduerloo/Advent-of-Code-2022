use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {

            let input_vec: Vec<char> = input.chars().collect();

            println!("⭐ First star ⭐ - found after {} characters", find_index(&input_vec, 4).unwrap());
            println!("🌟 Second star ✨ - found after {} characters", find_index(&input_vec, 14).unwrap());

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}

fn find_index(input_vec: &Vec<char>, size: usize) -> Option<usize> {
    for i in (size - 1)..input_vec.len() {
        let mut letters: HashMap<char, bool> = HashMap::new();

        for j in 0..size {
            let double = letters.entry(input_vec[i-j]).or_insert(false);
            *double = true;
        }

        if letters.len() == size {
            return Some(i + 1);
        } else {
            continue
        }
    }
    None
}