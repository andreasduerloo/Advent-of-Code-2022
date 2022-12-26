use std::collections::HashMap;

// Two HashMaps: name - flow rate, and name - neighbors. No pointers.

pub fn read_valve<'a>(line: &'a str, flowrates: &mut HashMap<&'a str, usize>, neighbors: &mut HashMap<&'a str, Vec<&'a str>>) {
    let split_line: Vec<&str> = line.split_whitespace().collect();

    flowrates.insert(split_line[1], usize::from_str_radix(&split_line[4][5..split_line[4].len() - 1], 10).unwrap());

    let mut neighbor_vec: Vec<&str> = Vec::new();

    // Neighbors start at index 9.

    let mut index: usize = 9;

    while index <= split_line.len() - 1 {
        if index == split_line.len() - 1 {
            // No comma after this one
            neighbor_vec.push(split_line[index]);
            break
        } else {
            // There's a comma to strip
            neighbor_vec.push(&split_line[index][0..split_line[index].len() - 1]);
            index += 1;
        }
    }

    neighbors.insert(split_line[1], neighbor_vec);

}