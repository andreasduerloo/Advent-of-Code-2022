use std::fs;
use std::env;
use day11_monkeys::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {

            let mut monkeys: Vec<Monkey> = Vec::new(); // Array initialization needs us to implement copy for Vectors

            input.split("\r\n\r\n").for_each(|val| monkeys.push(create_monkey(val)));

            // Secret sauce for second star: value mod (product of all the primes) will have the same moduloes as teh value itself
            // We can reduce big values to a more manageable size that way
            let mut magic_number: usize = 1;

            for i in 0..monkeys.len() {
                magic_number *= monkeys[i].div_by_test;
            }

            let mut star = 1;
            if let Some(value) = arguments.next() {
                if value == "2" {
                    star = 2;
                }
            }
            
            if star == 2 {
                for _i in 0..10000 {
                    for j in 0..8 {
                        inspect_and_throw(&mut monkeys, j, 2, &magic_number);
                    }
                }
            } else {
                for _i in 0..20 {
                    for j in 0..8 {
                        inspect_and_throw(&mut monkeys, j, 1, &magic_number);
                    }
                }
            }

            let mut monkey_business_scores: Vec<usize> = monkeys.into_iter().map(|m| m.thrown).collect();
            
            monkey_business_scores.sort();
            monkey_business_scores.reverse();

            let monkey_business: usize = monkey_business_scores.iter().take(2).product();

            if star == 2 {
                println!("🌟 Second star ✨ - Score after 10,000 rounds: {}", monkey_business);
            } else {
                println!("⭐ First star ⭐ - Score after 20 rounds: {}", monkey_business);
            }
        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
