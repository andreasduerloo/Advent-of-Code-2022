use std::fs;
use std::env;
use day12_hillclimbing::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {
            let mut grid: [[char; 114]; 41] = [['A'; 114]; 41];

            let lines: Vec<&str> = input.lines().collect();

            for i in 0..lines.len() {
                lines[i].chars()
                    .enumerate()
                    .for_each(|(j, c)| grid[i][j] = c);
            }

            // Find 'S'
            let mut start: Point = (0, 0);

            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid[i][j] == 'S' {
                        start = (i, j);
                    }
                }
            }

            // Find 'E'
            let mut goal: Point = (0, 0);

            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid[i][j] == 'E' {
                        goal = (i, j);
                    }
                }
            }

            let route_length: usize = a_star(start, goal, &grid);
            println!("Length of best route: {}", route_length);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}