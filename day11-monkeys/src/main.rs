use std::fs;
use std::env;
use day11_monkeys::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {
            
            let mut monkeys: Vec<Monkey> = Vec::new(); // Array initialization needs us to implement copy for Vectors

            input.split("\r\n\r\n").for_each(|val| monkeys.push(create_monkey(val)));

            for _i in 0..20 {
                for j in 0..8 {
                    inspect_and_throw(&mut monkeys, j);
                }
            }

            let mut monkey_business_scores: Vec<usize> = monkeys.into_iter().map(|m| m.thrown).collect();
            
            monkey_business_scores.sort();
            monkey_business_scores.reverse();

            let monkey_business: usize = monkey_business_scores.iter().take(2).product();

            println!("Score = {}", monkey_business);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
