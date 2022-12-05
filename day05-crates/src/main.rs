use std::fs;
use std::env;
use day05_crates::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {

            let input_vec: Vec<&str> = input.lines().collect();

            let mut config: Config = setup(input_vec);

            for instruction in config.instructions {
                apply_9000(instruction, &mut config.stack1);
                apply_9001(instruction, &mut config.stack2);
            }

            print!("⭐ First star ⭐ - top crates: ");

            for stack in config.stack1 {
                print!("{}", stack[stack.len() - 1]);
            }

            print!("\n🌟 Second star ✨ - top crates: ");
        
            for stack in config.stack2 {
                print!("{}", stack[stack.len() - 1]);
            }

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
