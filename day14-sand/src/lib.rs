pub type State = Vec<Vec<char>>;

pub fn build_rock(state: &mut State, instruction: &str) {
    let points: Vec<&str> = instruction.split(" -> ").collect(); // Now we have a vector of point pairs (as &str)
    
    let mut coordinates: Vec<(usize, usize)> = Vec::new();

    for point in points {
        let values: Vec<&str> = point.split(",").collect();
        coordinates.push((usize::from_str_radix(values[0], 10).unwrap(), usize::from_str_radix(values[1], 10).unwrap()))
    }

    let mut coord_iter = coordinates.into_iter().peekable();
    
    while let Some(mut current_point) = coord_iter.next() {
        while let Some(next_point) = coord_iter.peek() {
            // Find the direction
            if current_point.0 == next_point.0 { // Vertical
                if current_point.1 > next_point.1 { // Going up
                    for i in 0..((current_point.1 - next_point.1) + 1) {
                        state[current_point.1 - i][current_point.0] = '#';
                    }
                } else { // Going down
                    for i in 0..((next_point.1 - current_point.1) + 1) {
                        state[current_point.1 + i][current_point.0] = '#';
                    }
                }
            }
            else if current_point.1 == next_point.1 { // Horizontal
                if current_point.0 > next_point.0 { // Going left
                    for i in 0..((current_point.0 - next_point.0) + 1) {
                        state[current_point.1][current_point.0 - i] = '#';
                    }
                } else { // Going right
                    for i in 0..((next_point.0 - current_point.0) + 1) {
                        state[current_point.1][current_point.0 + i] = '#';
                    }
                }
            }
            current_point = coord_iter.next().unwrap();
        }
    }
}

pub enum Status {
    Moving((usize, usize)),
    Stable,
    Gone,
}

pub fn tick(state: &mut State, action: Status, counter: &mut usize, lowest: usize) -> (Status, bool) { //
    match action {
        Status::Gone => {
            println!("Sand has started falling into the abyss after {} grains.", counter);
            return (Status::Gone, true);
        },
        Status::Stable => { // The last grain of sand is stable, launch a new one
            *counter += 1;
            return (Status::Moving((500, 0)), false);
        },
        Status::Moving(location) => { // The current grain of sand is not yet stable
            // println!("Current location: {} {}", location.0, location.1);
            if location.1 > lowest {
                return (Status::Gone, true);
            }
            if state[location.1 + 1][location.0] == ' ' {
                return (Status::Moving((location.0, location.1 + 1)), false);
            }
            else { // The location below is blocked
                if state[location.1 + 1][location.0 - 1] == ' ' {
                    return (Status::Moving((location.0 - 1, location.1 + 1)), false);
                }
                else if state[location.1 + 1][location.0 + 1] == ' ' {
                    return (Status::Moving((location.0 + 1, location.1 + 1)), false);
                }
                else {
                    state[location.1][location.0] = '*';
                    return (Status::Stable, false);
                }
            } 
        }
    }
}

pub fn tick2(state: &mut State, action: Status, counter: &mut usize) -> (Status, bool) { // Run for the rectangle where the rock is, the triangles outside and on top can be calculated
    if state[0][500] == '#' {
        return (Status::Gone, true);
    }
    match action {
        Status::Gone => {
            println!("Sand has started falling into the abyss after {} grains.", counter);
            return (Status::Gone, true);
        },
        Status::Stable => { // The last grain of sand is stable, launch a new one
            *counter += 1;
            return (Status::Moving((500, 0)), false);
        },
        Status::Moving(location) => { // The current grain of sand is not yet stable
            if state[location.1 + 1][location.0] == ' ' {
                return (Status::Moving((location.0, location.1 + 1)), false);
            }
            else { // The location below is blocked
                if state[location.1 + 1][location.0 - 1] == ' ' {
                    return (Status::Moving((location.0 - 1, location.1 + 1)), false);
                }
                else if state[location.1 + 1][location.0 + 1] == ' ' {
                    return (Status::Moving((location.0 + 1, location.1 + 1)), false);
                }
                else {
                    state[location.1][location.0] = '*';
                    return (Status::Stable, false);
                }
            } 
        }
    }
}