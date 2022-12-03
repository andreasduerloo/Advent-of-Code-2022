use std::cmp::Ordering;
use std::collections::HashMap;

pub fn find_common(backpack: &str) -> Option<char> { // Assuming ONE incorrect item
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

pub fn item_value(item: char) -> u64 { // a - z = 1 - 26, A - Z = 27 - 52
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