use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let input_vec: Vec<&str> = input.lines().collect();

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

    println!("First star: the highest amount of calories is {}.", totals[totals.len()-1]);
    println!("Second star: the sum of the three highest calorie counts is {}.", totals[totals.len() - 1] + totals[totals.len() - 2] + totals[totals.len() - 3]);

}
