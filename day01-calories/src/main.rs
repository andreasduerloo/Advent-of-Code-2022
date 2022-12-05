use std::fs;
use std::env;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(content) = fs::read_to_string(filename) {

            let input_vec: Vec<&str> = content.lines().collect();
    
            let mut totals: Vec<usize> = Vec::new();
            let mut current: usize = 0;
        
            input_vec.iter().for_each( |line| {
                if *line != "" {
                    current += usize::from_str_radix(line, 10).unwrap();
                } else {
                    let total: usize = current;
                    totals.push(total);
                    current = 0;
                }
            });

            totals.sort();
    
            println!("⭐ First star ⭐ - The highest amount of calories is {}.", totals[totals.len()-1]);

            let top_three: usize = totals.drain(totals.len() - 3..).sum();

            println!("🌟 Second star ✨ - The sum of the three highest calorie counts is {}.", top_three);
        
        } else {
            eprintln!("Could not read file. Exiting. 🦌");    
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
