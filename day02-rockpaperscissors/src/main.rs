use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
    let ruleset: HashMap<&str, HashMap<&str, [usize; 2]>> = HashMap::from([
        ("X", HashMap::from([("A", [4, 3]), ("B", [1, 1]), ("C", [7, 2])])),
        ("Y", HashMap::from([("A", [8, 4]), ("B", [5, 5]), ("C", [2, 6])])),
        ("Z", HashMap::from([("A", [3, 8]), ("B", [9, 9]), ("C", [6, 7])]))
    ]);

    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(content) = fs::read_to_string(filename) {

            let mut input_vec: Vec<&str> = content.lines().collect();

            let mut scores: [usize; 2] = [0, 0];

            while let Some(combination) = input_vec.pop() {
                if let Some(score) = solve(combination, &ruleset, 0) {
                    scores[0] += score;
                }
                if let Some(score) = solve(combination, &ruleset, 1) {
                    scores[1] += score;
                }
            }

            println!("⭐ First star ⭐ - Total score is {}.", scores[0]);
            println!("🌟 Second star ✨ - Total score is {}.", scores[1]);
        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}

fn solve(input: &str, ruleset: &HashMap<&str, HashMap<&str, [usize; 2]>>, round: usize) -> Option<usize> {
    let moves: Vec<&str> = input.split_whitespace().collect();

    if let Some(rule) = ruleset.get(moves[1]) {
        if let Some(value) = rule.get(moves[0]) {
            Some(value[round])
        } else {
            None
        }
    } else {
        None
    }
}
