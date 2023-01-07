pub fn is_ordered(two_lines: &str) -> bool {
    let lines: Vec<&str> = two_lines.lines().collect();

    let mut chars: Vec<Vec<char>> = Vec::new();
    
    for i in 0..lines.len() {
        chars.push(lines[i].chars().collect());
    }

    let mut indices: [usize; 2] = [0, 0]; // Track an index for each line, they can move separately
    let mut list_depth: [usize; 2] = [0, 0]; // Also track how many lists 'deep' we are -> not currently used
    let mut added_depth: [usize; 2] = [0, 0]; // Check how often we had to pretend an int was a list
    let mut elements: [usize; 2] = [0, 0]; // Track whether one line has run out of elements

    while indices[0] < chars[0].len() || indices[1] < chars[1].len() {
        let mut compare: [bool; 2] = [false; 2]; // Check if we have something to compare

        // Left element
        if indices[0] < chars[0].len() {
            match chars[0][indices[0]] {
                '[' => { indices[0] += 1; list_depth[0] += 1; compare[0] = false },
                ']' => { indices[0] += 1; list_depth[0] -= 1; compare[0] = false },
                ',' => { indices[0] += 1; compare[0] = false },
                _ => { elements[0] += 1; compare[0] = true; } // Not entirely correct yet
            };
        } else {
            return true;
        }

        // Right element
        if indices[1] < chars[1].len() {
            match chars[1][indices[1]] {
                '[' => { indices[1] += 1; list_depth[1] += 1; compare[1] = false },
                ']' => { indices[1] += 1; list_depth[1] -= 1; compare[1] = false },
                ',' => { indices[1] += 1; compare[1] = false },
                _ => { elements[1] += 1; compare[1] = true; } // Not entirely correct yet
            };
        } else {
            return false;
        }
        
        // Compare if we have two numbers
        if compare[0] && compare[1] {
            let mut numbers: [u32; 2] = [0, 0];

            if chars[0][indices[0] + 1].is_ascii_digit() { // Not bothering with the index yet
                numbers[0] = 10 * chars[0][indices[0]].to_digit(10).unwrap() + chars[0][indices[0]+ 1].to_digit(10).unwrap();
            } else {
                numbers[0] = chars[0][indices[0]].to_digit(10).unwrap();
            }
            if chars[1][indices[1] + 1].is_ascii_digit() {
                numbers[1] = 10 * chars[1][indices[1]].to_digit(10).unwrap() + chars[1][indices[1]+ 1].to_digit(10).unwrap();
            } else {
                numbers[1] = chars[1][indices[1]].to_digit(10).unwrap();
            }

            if list_depth[1] > list_depth[0] {
                return true;
            }
            else if list_depth[0] > list_depth[1] {
                return false;
            }
            else if numbers[0] < numbers[1] {
                return true;
            }
            else if numbers[0] > numbers[1] {
                return false;
            }
            else {
                indices[0] += 1;
                indices[1] += 1;
            }
        }
        else if compare[0] { // Does this play nice with commas?
            list_depth[0] += 1;
            added_depth[0] += 1;
        }
        else if compare[1] {
            list_depth[1] += 1;
            added_depth[1] += 1;
        }
    }

    // The loop is done - one line ran out
    println!("Element counts: {} and {}", elements[0], elements[1]);
    if elements[1] > elements[0] {
        return true;
    } else {
        false
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let input = "[1,1,3,1,1]\r\n[1,1,5,1,1]";

        assert_eq!(is_ordered(input), true);
    }

    #[test]
    fn second_example() {
        let input = "[[1],[2,3,4]]\r\n[[1],4]";

        assert_eq!(is_ordered(input), true);
    }

    #[test]
    fn third_example() {
        let input = "[9]\r\n[[8,7,6]]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn fourth_example() {
        let input = "[[4,4],4,4]\r\n[[4,4],4,4,4]";

        assert_eq!(is_ordered(input), true);
    }

    #[test]
    fn fifth_example() {
        let input = "[7,7,7,7]\r\n[7,7,7]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn sixth_example() {
        let input = "[]\r\n[3]";

        assert_eq!(is_ordered(input), true);
    }

    #[test]
    fn seventh_example() {
        let input = "[[[]]]\r\n[[]]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn eighth_example() {
        let input = "[1,[2,[3,[4,[5,6,7]]]],8,9]\r\n[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(is_ordered(input), false);
    }

    #[test]
    fn test_with_10() {
        let input = "[2]\r\n[10]";

        assert_eq!(is_ordered(input), true);
    }
}