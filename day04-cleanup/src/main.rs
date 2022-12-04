use std::fs;
use std::env;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(content) = fs::read_to_string(filename) {
            
            let mut input_vec: Vec<&str> = content.lines().collect();

            let mut overlap: [usize; 2] = [0, 0];

            while let Some(instruction) = input_vec.pop() {
                let coordinates: [usize; 4] = parse_instruction(instruction);

                if fully_contains(&coordinates) { // No need to check the second part
                    overlap[0] += 1;
                    overlap[1] += 1;
                }
                else if overlaps(&coordinates) {
                    overlap[1] += 1;
                }
            }

            println!("⭐ First star ⭐ - {} instructions fully overlap.", overlap[0]);
            println!("🌟 Second star ✨ - {} instructions overlap at least a little.", overlap[1]);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}

fn parse_instruction(instructions: &str) -> [usize; 4] {
    let split_instructions: Vec<&str> = instructions.split(&[',','-'][..]).collect();

    let mut output: [usize; 4] = [0; 4];

    for i in 0..4 {
        output[i] = usize::from_str_radix(&split_instructions[i], 10).unwrap();
    }

    output
}

fn fully_contains(coordinates: &[usize; 4]) -> bool {
    if ( coordinates[0] <= coordinates[2] && coordinates[1] >= coordinates[3] ) || ( coordinates[0] >= coordinates[2] && coordinates[1] <= coordinates[3] ) {
        true
    } else {
        false
    }
}

fn overlaps(coordinates: &[usize; 4]) -> bool {
    if ( coordinates[0] <= coordinates[2] && coordinates[1] >= coordinates[2] ) || ( coordinates[0] >= coordinates[2] && coordinates[0] <= coordinates[3] ) {
        true
    } else {
        false
    }
}
