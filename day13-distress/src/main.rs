use std::fs;
use std::env;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {
            
            let solution: usize = input.split("\r\n\r\n")
                .enumerate()
                .map(|(i, val)| (i, is_ordered(val)))
                .filter(|tup| tup.1)
                .map(|tup| tup.0)
                .reduce(|acc, val| acc + val)
                .unwrap();

            println!("Solution: {}", solution);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}

fn is_ordered(two_lines: &str) -> bool {
    let lines: Vec<&str> = two_lines.lines().collect();

    // Let's try ignoring brackets
    let left_line: Vec<char> = lines[0].chars().filter(|c| c.is_ascii_digit()).collect();
    let right_line: Vec<char> = lines[1].chars().filter(|c| c.is_ascii_digit()).collect();

    if left_line.len() == 0 {
        return true;
    }
    else if right_line.len() == 0 {
        return false;
    }

    for i in 0..left_line.len() {
        if left_line[i] > right_line[i] {
            return false;
        }
        else if i == right_line.len() - 1 {
            return false;
        }
    }
    true
}
