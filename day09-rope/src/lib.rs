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

pub fn move_head2(instruction: (&str, isize), rope: &mut [Point; 10], map: &mut Map) {
    match instruction.0 {
        "D" => {
            for _i in 0..instruction.1 {
                rope[0].1 -= 1;
                for j in 1..10 {
                    move_tail2(rope, j);
                    if j == 9 {
                        map.entry(rope[9]).or_insert(true);
                    }
                }
            }
        },
        "L" => {
            for _i in 0..instruction.1 {
                rope[0].0 -= 1;
                for j in 1..10 {
                    move_tail2(rope, j);
                    if j == 9 {
                        map.entry(rope[9]).or_insert(true);
                    }
                }
            }
        },
        "R" => {
            for _i in 0..instruction.1 {
                rope[0].0 += 1;
                for j in 1..10 {
                    move_tail2(rope, j);
                    if j == 9 {
                        map.entry(rope[9]).or_insert(true);
                    }
                }
            }
        },
        "U" => {
            for _i in 0..instruction.1 {
                rope[0].1 += 1;
                for j in 1..10 {
                    move_tail2(rope, j);
                    if j == 9 {
                        map.entry(rope[9]).or_insert(true);
                    }
                }
            }
        },
        _ => {
            return
        }
    }
}

fn move_tail2(rope: &mut [Point; 10], index: usize) {
    // Decide if we need to move - either dimension is two higher or lower
    if (rope[index - 1].0 - rope[index].0) > 1 || (rope[index - 1].0 - rope[index].0) < (-1) || (rope[index - 1].1 - rope[index].1) > 1 || (rope[index - 1].1 - rope[index].1) < (-1) {
        // Move the tail
        // Horizontally
        if rope[index - 1].1 == rope[index].1 {
            if rope[index - 1].0 < rope[index].0 { // The head is to the left
                rope[index].0 -= 1;
            } else {
                rope[index].0 += 1;
            }
        }
        // Vertically
        else if rope[index - 1].0 == rope[index].0 {
            if rope[index - 1].1 < rope[index].1 { // The head is below the tail
                rope[index].1 -= 1;
            } else {
                rope[index].1 += 1;
            }
        }
        // Diagonally
        else {
            if rope[index - 1].0 > rope[index].0 && rope[index - 1].1 > rope[index].1 { // Top right
                rope[index].0 += 1;
                rope[index].1 += 1;
            }
            else if rope[index - 1].0 > rope[index].0 && rope[index - 1].1 < rope[index].1 { // Bottom right
                rope[index].0 += 1;
                rope[index].1 -= 1;
            }
            else if rope[index - 1].0 < rope[index].0 && rope[index - 1].1 > rope[index].1 { // Top left
                rope[index].0 -= 1;
                rope[index].1 += 1;
            }
            else { // Bottom left
                rope[index].0 -= 1;
                rope[index].1 -= 1;
           }
        }
    }  
}