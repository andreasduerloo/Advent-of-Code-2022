use std::fs;
use std::env;

type Point = (usize, usize);

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
            let mut end: Point = (0, 0);

            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid[i][j] == 'S' {
                        start = (i, j);
                    }
                }
            }

            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid[i][j] == 'E' {
                        end = (i, j);
                    }
                }
            }

            // println!("Start is at ({}, {})", start.0, start.1);

            let visited: Vec<Point> = Vec::new();
            // find_path(end, &grid, 0, visited);
            find_path_reverse(end, &grid, 0)

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}

fn find_path(position: Point, grid: &mut [[char; 114]; 41], counter: usize, visited: Vec<Point>) {
    if grid[position.0][position.1] == 'E' {
        println!("Made it to E after {} steps!", counter);
    }

    else {
        // We're not there yet, start again from eligible neighbors

        // We can go one up, stay the same, or go down - let's not go down
        // Look right
        if position.1 != 113 {
            if u64::from(grid[position.0][position.1 + 1]) == u64::from(grid[position.0][position.1])
            || u64::from(grid[position.0][position.1 + 1]) == u64::from(grid[position.0][position.1]) + 1
            || grid[position.0][position.1] == 'S' {
                if !visited.contains(&position) {
                    let mut next_contains: Vec<Point> = visited.clone();
                    next_contains.push(position); // Flag the current position as visited
                    find_path((position.0, position.1 + 1), grid, counter + 1, next_contains);
                }
                
            }
        }

        // Look up
        if position.0 != 0 {
            if u64::from(grid[position.0 - 1][position.1]) == u64::from(grid[position.0][position.1]) 
            || u64::from(grid[position.0 - 1][position.1]) == u64::from(grid[position.0][position.1]) + 1 
            || grid[position.0][position.1] == 'S' {
                if !visited.contains(&position) {
                    let mut next_contains: Vec<Point> = visited.clone();
                    next_contains.push(position);
                    find_path((position.0 - 1, position.1), grid, counter + 1, next_contains);
                }
            }
        }

        // Look down
        if position.0 != 40 {
            if u64::from(grid[position.0 + 1][position.1]) == u64::from(grid[position.0][position.1])
            || u64::from(grid[position.0 + 1][position.1]) == u64::from(grid[position.0][position.1]) + 1 
            || grid[position.0][position.1] == 'S' {
                if !visited.contains(&position) {
                    let mut next_contains: Vec<Point> = visited.clone();
                    next_contains.push(position);
                    find_path((position.0 + 1, position.1), grid, counter + 1, next_contains);
                }
            }
        }

        // Look left
        if position.1 != 0 {
            if u64::from(grid[position.0][position.1 - 1]) == u64::from(grid[position.0][position.1])
            || u64::from(grid[position.0][position.1 - 1]) == u64::from(grid[position.0][position.1]) + 1
            || grid[position.0][position.1] == 'S' {
                if !visited.contains(&position) {
                    let mut next_contains: Vec<Point> = visited.clone();
                    next_contains.push(position);
                    find_path((position.0, position.1 - 1), grid, counter + 1, next_contains);
                }
            }
        }
    }   
}

fn find_path_reverse(position: Point, grid: &[[char; 114]; 41], counter: usize) {
    if grid[position.0][position.1] == 'S' {
        println!("Made it to S after {} steps!", counter);
    }

    else {
        // We're not there yet, start again from eligible neighbors

        // We can go one down or stay the same
        // Look right
        if position.1 != 113 {
            if u64::from(grid[position.0][position.1 + 1]) >= u64::from(grid[position.0][position.1]) - 1
            || grid[position.0][position.1] == 'E' {
                find_path_reverse((position.0, position.1 + 1), grid, counter + 1);
            }
        }

        // Look up
        if position.0 != 0 {
            if u64::from(grid[position.0 - 1][position.1]) >= u64::from(grid[position.0][position.1]) + 1 
            || grid[position.0][position.1] == 'E' {
                find_path_reverse((position.0 - 1, position.1), grid, counter + 1);
            }
        }

        // Look down
        if position.0 != 40 {
            if u64::from(grid[position.0 + 1][position.1]) >= u64::from(grid[position.0][position.1]) - 1 
            || grid[position.0][position.1] == 'E' {
                find_path_reverse((position.0 + 1, position.1), grid, counter + 1);
            }
        }

        // Look left
        if position.1 != 0 {
            if u64::from(grid[position.0][position.1 - 1]) >= u64::from(grid[position.0][position.1]) - 1
            || grid[position.0][position.1] == 'E' {
                find_path_reverse((position.0, position.1 - 1), grid, counter + 1);
            }
        }
    }   
}