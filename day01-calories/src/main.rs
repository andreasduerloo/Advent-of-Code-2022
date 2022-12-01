use std::fs;
use std::env;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(content) = fs::read_to_string(filename) {

            let input_vec: Vec<&str> = content.lines().collect();
    
            let mut totals: Vec<usize> = Vec::new();
            let mut current: usize = 0;
        
            for line in input_vec {
                if line != "" {
                    current += usize::from_str_radix(line, 10).unwrap();
                }
                else {
                    let total: usize = current;
                    totals.push(total);
                    current = 0;
                }
            }
            totals.sort();
    
            println!("⭐ First star ⭐ - The highest amount of calories is {}.", totals[totals.len()-1]);
            println!("🌟 Second star ✨ - The sum of the three highest calorie counts is {}.", totals[totals.len() - 1] + totals[totals.len() - 2] + totals[totals.len() - 3]);
        
        } else {
            println!("Could not read file. Exiting. 🦌");    
        }
    } else {
        println!("No argument was passed. Exiting. 🦌");
    }
}
