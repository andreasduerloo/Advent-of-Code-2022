struct Valve {
    name: &str,
    flow_rate: usize,
    // neighbors: Vec<&Valve>
    neighbors: Vec<Box<Valve>>
}

// Other option: Valve contains the name of its neighbors, and we have a HashMap with the flow rates per valve.

pub fn read_valve(line: &str) -> Valve {
    let split_line: Vec<&str> = line.split_whitespace().collect();

    Valve {
        name: split_line[1],
        flow_rate: usize::from_str_radix(split_line[4][5..split_line[4].len() - 1], 10).unwrap();
        // neighbors:
    }
}