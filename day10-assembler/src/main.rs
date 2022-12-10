use std::fs;
use std::env;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {

            let mut values: Vec<isize> = vec![1];
            let mut value: isize = 1;

            input.lines().for_each(|instr| {
                execute(instr, &mut values, &mut value);
            });

            println!("Result: {}", values[20] * 20 + values[60] * 60 + values[100] * 100 + values[140] * 140 + values[180] * 180 + values[220] * 220);

            for i in 0..6 {
                for j in 0..40 {
                    if values[(i * 40) + j + 1] == j as isize || values[(i * 40) + j + 1] == (j as isize + 1) || values[(i * 40) + j + 1] == (j as isize - 1) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}

fn execute(instruction: &str, values: &mut Vec<isize>, current_value: &mut isize) {
    let instruction_split: Vec<&str> = instruction.split_whitespace().collect();

    match instruction_split[0] {
        "noop" => {
            values.push(*current_value);
            
        },
        "addx" => {
            let value = isize::from_str_radix(instruction_split[1], 10).unwrap();
            for _i in 0..2 {
                values.push(*current_value);
            }
            *current_value += value;
        },
        _ => return
    }
}