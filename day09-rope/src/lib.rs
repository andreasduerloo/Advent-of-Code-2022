use std::collections::HashMap;

pub type Point = (isize, isize);
pub type Map = HashMap<Point, bool>;

// enum direction?

pub fn parse(instruction: &str) -> (&str, isize) {
    let pair: Vec<&str> = instruction.split_whitespace().collect();
    let direction: &str = pair[0];
    let distance: isize = isize::from_str_radix(pair[1], 10).unwrap();

    (direction, distance)    
}

pub fn move_rope(instruction: (&str, isize), rope: &mut [Point], map: &mut Map) {
    for _i in 0..instruction.1 {
        match instruction.0 {
            "D" => {
                rope[0].1 -= 1;
            },
            "L" => {
                rope[0].0 -= 1;
            },
            "R" => {
                rope[0].0 += 1;
            },
            "U" => {
                rope[0].1 += 1;
            },
            _ => return
        }
        follow_head(rope, map);
    }
}

fn follow_head(rope: &mut [Point], map: &mut Map) {
    for i in 1..rope.len() { // For each knot, one after the other
        if (rope[i - 1].0 - rope[i].0) > 1 || (rope[i - 1].0 - rope[i].0) < (-1) || (rope[i - 1].1 - rope[i].1) > 1 || (rope[i - 1].1 - rope[i].1) < (-1) {
            // We have to move this particular knot
            // Horizontally
            if rope[i - 1].1 == rope[i].1 {
                if rope[i - 1].0 < rope[i].0 { // The previous knot is to the left
                    rope[i].0 -= 1;
                } else {
                    rope[i].0 += 1;
                }
            }
            // Vertically
            else if rope[i - 1].0 == rope[i].0 {
                if rope[i - 1].1 < rope[i].1 { // The head is below the tail
                    rope[i].1 -= 1;
                } else {
                    rope[i].1 += 1;
                }
            }
            else {
                if rope[i - 1].0 > rope[i].0 && rope[i - 1].1 > rope[i].1 { // Top right
                    rope[i].0 += 1;
                    rope[i].1 += 1;
                }
                else if rope[i - 1].0 > rope[i].0 && rope[i - 1].1 < rope[i].1 { // Bottom right
                    rope[i].0 += 1;
                    rope[i].1 -= 1;
                }
                else if rope[i - 1].0 < rope[i].0 && rope[i - 1].1 > rope[i].1 { // Top left
                    rope[i].0 -= 1;
                    rope[i].1 += 1;
                }
                else { // Bottom left
                    rope[i].0 -= 1;
                    rope[i].1 -= 1;
               }
            }
            if i == rope.len() - 1 { // We just moved the tail
                map.entry(rope[i]).or_insert(true);
            }
        } else { // The previous knot has not moved, we can break out of the entire loop - store the tail's location anyway to get the origin
            map.entry(rope[rope.len() - 1]).or_insert(true);
            return
        }
    }
}
