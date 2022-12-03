use std::fs;
use std::env;
use std::cmp::Ordering;
use std::collections::HashMap;

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

fn find_common(backpack: &str) -> Option<char> { // Assuming ONE incorrect item
    let char_vec: Vec<char> = backpack.chars().collect();

    let mut first_compartment: HashMap<char, usize> = HashMap::new();

    for i in 0..char_vec.len()/2 { // Store what items we have in the first compartment (and how many, just in case)
        let count = first_compartment.entry(char_vec[i]).or_insert(0);
        *count += 1;
    }

    for i in char_vec.len()/2..char_vec.len() { // Look for the first item that has an entry in the HashMap
        if let Some(_value) = first_compartment.get(&char_vec[i]) {
            return Some(char_vec[i]);
        }
    }

    None
}

fn item_value(item: char) -> u64 { // a - z = 1 - 26, A - Z = 27 - 52
    let bytes = u64::from(item);

    match bytes.cmp(&0x60) {
        Ordering::Greater => { // Lowercase
            bytes - 0x60
        },
        _ => { // Uppercase
            bytes - 0x26
        }
    }
}