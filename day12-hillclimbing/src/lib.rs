use std::collections::{VecDeque, HashMap};

pub type Point = (usize, usize);
pub type Grid = [[(char, bool); 114]; 41];

pub fn breadth_first_search(grid: &mut Grid, start: Point) -> usize {
    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut parents: HashMap<Point, Point> = HashMap::new();

    grid[start.0][start.1].1 = true;

    queue.push_back(start);

    while let Some(point) = queue.pop_front() {
        if grid[point.0][point.1].0 == 'E' {
            return count_path_len(parents, point);
        }

        // Build neighbor Vec
        let mut neighbors: Vec<Point> = Vec::new();

        // Look right
        if point.1 != 113 {
            if u64::from(grid[point.0][point.1 + 1].0) <= u64::from(grid[point.0][point.1].0) + 1
            || grid[point.0][point.1].0 == 'S' {
                neighbors.push((point.0, point.1 + 1));
            }
        }

        // Look up
        if point.0 != 0 {
            if u64::from(grid[point.0 - 1][point.1].0) <= u64::from(grid[point.0][point.1].0) + 1
            || grid[point.0][point.1].0 == 'S' {
                neighbors.push((point.0 - 1, point.1));
            }
        }

        // Look down
        if point.0 != 40 {
            if u64::from(grid[point.0 + 1][point.1].0) <= u64::from(grid[point.0][point.1].0) + 1 
            || grid[point.0][point.1].0 == 'S' {
                neighbors.push((point.0 + 1, point.1));
            }
        }

        // Look left
        if point.1 != 0 {
            if u64::from(grid[point.0][point.1 - 1].0) <= u64::from(grid[point.0][point.1].0) + 1
            || grid[point.0][point.1].0 == 'S' {
                neighbors.push((point.0, point.1 - 1));
            }
        }

        for neighbor in neighbors {
            if !grid[neighbor.0][neighbor.1].1 {
                grid[neighbor.0][neighbor.1].1 = true;
                parents.insert((neighbor.0, neighbor.1), (point.0, point.1));
                queue.push_back(neighbor);
            }
        }
    }
    0
}

fn count_path_len(parents: HashMap<Point, Point>, mut point: Point) -> usize {
    let mut out: usize = 0;

    while let Some(parent) = parents.get(&point) {
        out += 1;
        point = *parent;
    }
    out
}