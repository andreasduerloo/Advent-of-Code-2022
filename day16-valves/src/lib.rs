struct Valve {
    name: &str,
    flow_rate: usize,
    // neighbors: Vec<&Valve>
    neighbors: Vec<Box<Valve>>
}

// Other option: Valve contains the name of its neighbors, and we have a HashMap with the flow rates per valve.

pub fn read_valve(line: &str) -> Valve {
    //

    Valve {
        //
    }
}