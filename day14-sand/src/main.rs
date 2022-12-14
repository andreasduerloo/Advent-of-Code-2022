use std::fs;
use std::env;
use day14_sand::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {

            // Get the highest column number, as well as the higest row number
            let lines: Vec<&str> = input.lines().collect();

            // Create a state
            let mut state: State = Vec::new();

            for i in 0..200 {
                state.push(Vec::new());
                for _j in 0..1000 {
                    state[i].push(' ');
                }
            }

            for line in lines {
                build_rock(&mut state, line);
            }

            let mut lowest: usize = 0;

            for i in 1..200 {
                if state[200 - i].contains(&'#') {
                    lowest = 200 - i;
                    break
                }
            }

            let mut counter: usize = 0;

            let mut tick_result = tick(&mut state, Status::Moving((500, 0)), &mut counter, lowest);

            while !tick_result.1 {
                tick_result = tick(&mut state, tick_result.0, &mut counter, lowest);
            }

            println!("Sand has started falling into the abyss after {} grains.", counter);

            // Reset the state
            for i in 0..200 {
                for j in 0..1000 {
                    state[i][j] = ' ';
                }
            }

            // Draw the bottom

            for i in 0..1000 {
                state[lowest + 2][i] = '#';
            }

            counter = 0;

            let mut tick_result = tick2(&mut state, Status::Moving((500, 0)), &mut counter);

            while !tick_result.1 {
                tick_result = tick2(&mut state, tick_result.0, &mut counter);
            }

            println!("The source is blocked after {} grains.", counter);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
