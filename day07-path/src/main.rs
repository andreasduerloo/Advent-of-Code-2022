use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {
            
            let mut current_path: Vec<&str> = Vec::new();
            let mut directories: HashMap<Vec<&str>, usize> = HashMap::new();

            for line in input.lines() {
                parse(line, &mut current_path, &mut directories);
            }

            let to_free: usize = 30000000 - (70000000 - directories.get(&Vec::from(["/"])).unwrap());
            let mut closest_entry: usize = 70000000;

            // let total_under: usize = directories.values() // Can't put a 'for_each' in between here, vector is consumed.
            //     .filter(|size| { *size <= &100000 })
            //     .sum();

            let mut total_under: usize = 0;

            for size in directories.values() {
                if size > &to_free && size < &closest_entry {
                    closest_entry = *size;
                }
                if size < &100000 {
                    total_under += *size;
                }
            }

            println!("⭐ First star ⭐ - total size of directories under 100000 in size: {}", total_under);
            println!("🌟 Second star ✨ - size of best directory to delete: {}",  closest_entry);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}

fn parse<'a>(command: &'a str, path: &mut Vec<&'a str>, dirs: &mut HashMap<Vec<&'a str>, usize>) {
    let split_line: Vec<&str> = command.split_whitespace().collect();

    match split_line[0] {
        "$" => {
            if split_line[1] == "cd" {
                if split_line[2] == ".." {
                    let _ = path.pop();
                } else {
                    path.push(split_line[2]);
                }
            } // We ignore 'ls'
        },
        "dir" => { // Not useful
            return
        }
        _ => { // This is a size, increase all directories in the current path
            for i in 0..path.len() {
                let size: usize = usize::from_str_radix(split_line[0], 10).unwrap();

                dirs.entry(path[0..(path.len() - i)].to_vec())
                    .and_modify(|s| *s += size)
                    .or_insert(size);
            }
        }
    }
}
