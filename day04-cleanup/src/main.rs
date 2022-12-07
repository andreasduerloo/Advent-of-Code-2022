use std::fs;
use std::env;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(content) = fs::read_to_string(filename) {
            
            // let mut input_vec: Vec<&str> = content.lines().collect();

            // let mut overlap: [usize; 2] = [0, 0];

            // while let Some(instruction) = input_vec.pop() {
            //     let coordinates: [usize; 4] = parse_instruction(instruction);

            //     if fully_contains(&coordinates) { // No need to check the second part
            //         overlap[0] += 1;
            //         overlap[1] += 1;
            //     }
            //     else if overlaps(&coordinates) {
            //         overlap[1] += 1;
            //     }
            // }

            let overlap: (usize, usize) = content.lines()
                .map(|s| parse_instruction(s))
                .map(|c| (fully_contains(&c), overlaps(&c)))
                .reduce(|total, item| {
                    (total.0 + item.0, total.1 + item.1) })
                .unwrap();

            println!("⭐ First star ⭐ - {} instructions fully overlap.", overlap.0);
            println!("🌟 Second star ✨ - {} instructions overlap at least a little.", overlap.1);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}

fn parse_instruction(instructions: &str) -> [usize; 4] {
    let mut output: [usize; 4] = [0; 4];

    instructions.split(&[',','-'][..])
        .enumerate()
        .for_each( |(i, val)| { output[i] = usize::from_str_radix(val, 10).unwrap(); });

    output
}

fn fully_contains(coordinates: &[usize; 4]) -> usize // bool {
    if ( coordinates[0] <= coordinates[2] && coordinates[1] >= coordinates[3] ) || ( coordinates[0] >= coordinates[2] && coordinates[1] <= coordinates[3] ) {
        1 // true
    } else {
        0 // false
    }
}

fn overlaps(coordinates: &[usize; 4]) -> usize // bool {
    if ( coordinates[0] <= coordinates[2] && coordinates[1] >= coordinates[2] ) || ( coordinates[0] >= coordinates[2] && coordinates[0] <= coordinates[3] ) {
        1 // true
    } else {
        0 // false
    }
}
