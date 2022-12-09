use std::collections::HashMap;

pub type Point = (isize, isize);
pub type Map = HashMap<Point, bool>;

pub fn parse(instruction: &str) -> (&str, isize) {
    let pair: Vec<&str> = instruction.split_whitespace().collect();
    let direction: &str = pair[0];
    let distance: isize = isize::from_str_radix(pair[1], 10).unwrap();

    (direction, distance)    
}

pub fn move_head(instruction: (&str, isize), head: &mut Point, tail: &mut Point, map: &mut Map) {
    match instruction.0 {
        "D" => {
            for _i in 0..instruction.1 {
                head.1 -= 1;
                move_tail(&(head.0, head.1), tail, map);
            }
        },
        "L" => {
            for _i in 0..instruction.1 {
                head.0 -= 1;
                move_tail(&(head.0, head.1), tail, map);
            }
        },
        "R" => {
            for _i in 0..instruction.1 {
                head.0 += 1;
                move_tail(&(head.0, head.1), tail, map);
            }
        },
        "U" => {
            for _i in 0..instruction.1 {
                head.1 += 1;
                move_tail(&(head.0, head.1), tail, map);
            }
        },
        _ => {
            return
        }
    }
}

fn move_tail(head: &Point, tail: &mut Point, map: &mut Map) {
    // Decide if we need to move - either dimension is two higher or lower
    if (head.0 - tail.0) > 1 || (head.0 - tail.0) < (-1) || (head.1 - tail.1) > 1 || (head.1 - tail.1) < (-1) {
        // Move the tail
        // Horizontally
        if head.1 == tail.1 {
            if head.0 < tail.0 { // The head is to the left
                tail.0 -= 1;
            } else {
                tail.0 += 1;
            }
        }
        // Vertically
        else if head.0 == tail.0 {
            if head.1 < tail.1 { // The head is below the tail
                tail.1 -= 1;
            } else {
                tail.1 += 1;
            }
        }
        // Diagonally
        else {
            if head.0 > tail.0 && head.1 > tail.1 { // Top right
                tail.0 += 1;
                tail.1 += 1;
            }
            else if head.0 > tail.0 && head.1 < tail.1 { // Bottom right
                tail.0 += 1;
                tail.1 -= 1;
            }
            else if head.0 < tail.0 && head.1 > tail.1 { // Top left
                tail.0 -= 1;
                tail.1 += 1;
            }
            else { // Bottom left
                tail.0 -= 1;
                tail.1 -= 1;
            }
        }
    }
    // Mark current position
    map.entry(*tail).or_insert(true);
}