use std::fs;
use std::env;
use day12_hillclimbing::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {
            let mut grid: Grid = [[('A', false); 114]; 41];

            let lines: Vec<&str> = input.lines().collect();

            for i in 0..lines.len() {
                lines[i].chars()
                    .enumerate()
                    .for_each(|(j, c)| {
                        grid[i][j].0 = c;
                    });
            }

            // Find 'S'
            let mut start: Point = (0, 0);

            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid[i][j].0 == 'S' {
                        start.0 = i;
                        start.1 = j;
                    }
                }
            }

            let route_length: usize = breadth_first_search(&mut grid, start);

            println!("Length of best route: {}", route_length);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}