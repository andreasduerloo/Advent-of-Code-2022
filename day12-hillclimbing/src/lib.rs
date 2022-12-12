// Save me, Wikipedia
// https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode

use std::collections::HashSet;
use std::collections::HashMap;

pub type Point = (usize, usize);

pub fn a_star(start: Point, goal: Point, grid: &[[char; 114]; 41]) -> usize {
    let mut lead_nodes: HashSet<Point> = HashSet::new();
    let _ = lead_nodes.insert(start);

    let mut came_from: HashMap<Point, Point> = HashMap::new();

    let mut lengths: HashMap<Point, usize> = HashMap::new();
    lengths.insert(start, 0);

    let mut best_distances: HashMap<Point, usize> = HashMap::new();
    best_distances.insert(start, direct_distance(&start, &goal));


    while lead_nodes.len() != 0 {
        let current_position = find_closest(&lead_nodes, &best_distances); // What happens when there's a draw?

        if current_position == goal {
            return path_length(came_from, current_position);
        }

        let _ = lead_nodes.remove(&current_position);

        // Build vec with neighbors

        let mut neighbors: Vec<Point> = Vec::new();

        // Look right
        if current_position.1 != 113 {
            if u64::from(grid[current_position.0][current_position.1 + 1]) <= u64::from(grid[current_position.0][current_position.1]) + 1
            || grid[current_position.0][current_position.1] == 'S' {
                neighbors.push((current_position.0, current_position.1 + 1));
            }
        }

        // Look up
        if current_position.0 != 0 {
            if u64::from(grid[current_position.0 - 1][current_position.1]) <= u64::from(grid[current_position.0][current_position.1]) + 1
            || grid[current_position.0][current_position.1] == 'S' {
                neighbors.push((current_position.0 - 1, current_position.1));
            }
        }

        // Look down
        if current_position.0 != 40 {
            if u64::from(grid[current_position.0 + 1][current_position.1]) <= u64::from(grid[current_position.0][current_position.1]) + 1 
            || grid[current_position.0][current_position.1] == 'S' {
                neighbors.push((current_position.0 + 1, current_position.1));
            }
        }

        // Look left
        if current_position.1 != 0 {
            if u64::from(grid[current_position.0][current_position.1 - 1]) <= u64::from(grid[current_position.0][current_position.1]) + 1
            || grid[current_position.0][current_position.1] == 'S' {
                neighbors.push((current_position.0, current_position.1 - 1));
            }
        }

        for neighbor in neighbors {
            let attempt_length: usize = lengths.get(&current_position).unwrap() + 1;
            if let Some(known_length) = lengths.get(&neighbor) { // We know a route from start to here
                if attempt_length < *known_length { // We found a shorter route
                    came_from.insert(neighbor, current_position);
                    lengths.insert(neighbor, attempt_length);
                    best_distances.insert(neighbor, attempt_length + direct_distance(&neighbor, &goal));

                    if !lead_nodes.contains(&neighbor) {
                        let _ = lead_nodes.insert(neighbor);
                    }
                }
            }
            else { // We didn't know a route yet
                came_from.insert(neighbor, current_position);
                lengths.insert(neighbor, attempt_length);
                best_distances.insert(neighbor, attempt_length + direct_distance(&neighbor, &goal));

                if !lead_nodes.contains(&neighbor) {
                    let _ = lead_nodes.insert(neighbor);
                }
            }
        }
    }
    0 // Something went wrong
}

fn direct_distance(start: &Point, destination: &Point) -> usize {
    let mut distance: usize = 0;

    if destination.0 > start.0 {
        distance += destination.0 - start.0;
    }
    else if start.0 > destination.0 {
        distance += start.0 - destination.0;
    }

    if destination.1 > start.1 {
        distance += destination.1 - start.1;
    }
    else if start.1 > destination.1 {
        distance += start.1 - destination.1;
    }
    distance
}

fn find_closest(candidates: &HashSet<Point>, distances: &HashMap<Point, usize>) -> Point {
    let mut local_map: HashMap<Point, usize> = HashMap::new();

    for candidate in candidates {
        local_map.insert(*candidate, *distances.get(candidate).unwrap());
    }

    let best_point = local_map.keys().max_by_key(|k| k.1).unwrap();
    *best_point
}

fn path_length(table: HashMap<Point, Point>, mut current: Point) -> usize {
    let mut output: usize = 0;

    while let Some(previous) = table.get(&current) {
        current = *previous;
        output += 1;
    }
    output
}