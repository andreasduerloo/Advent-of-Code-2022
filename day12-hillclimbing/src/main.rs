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

            let mut grid2 = grid.clone(); // We'll need this later

            let (route_length, e) = breadth_first_search(&mut grid, start, 'E'); // Also tells us where 'E' is

            println!("⭐ First star ⭐ - Length of best route: {}", route_length);
            println!("🌟 Second star ✨ - Shortest route to an 'a': {}", breadth_first_descend(&mut grid2, e, 'a').0);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}