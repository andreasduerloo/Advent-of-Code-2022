use std::fs;
use std::env;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(content) = fs::read_to_string(filename) {
            
            let overlap: (usize, usize) = content.lines()
                .map(|s| overlap(&parse_instruction(s)))
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

fn overlap(coordinates: &[usize; 4]) -> (usize, usize) {
    if ( coordinates[0] <= coordinates[2] && coordinates[1] >= coordinates[3] ) || ( coordinates[0] >= coordinates[2] && coordinates[1] <= coordinates[3] ) {
        (1, 1) // Complete overlap
    }
    else if ( coordinates[0] <= coordinates[2] && coordinates[1] >= coordinates[2] ) || ( coordinates[0] >= coordinates[2] && coordinates[0] <= coordinates[3] ) {
        (0, 1) // Partial overlap
    } else {
        (0, 0)
    }
}
