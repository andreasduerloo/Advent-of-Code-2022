use std::collections::HashSet;

// X is the column, Y is the row
pub struct Sensor {
    pub x: isize,
    pub y: isize,
    pub distance_to_closest: usize
}

pub fn create_sensor(input: &str, beacons_on_line: &mut HashSet<isize>) -> Sensor {
    let input_vec: Vec<&str> = input.split_whitespace().collect();

    let mut coordinates: [isize; 4] = [0; 4];
    let indices: [usize; 4] = [2, 3, 8, 9];

    for i in 0..4 {
        match indices[i] {
            9 => {
                coordinates[i] = isize::from_str_radix(&input_vec[indices[i]][2..], 10).unwrap();
            },
            _ => {
                coordinates[i] = isize::from_str_radix(&input_vec[indices[i]][2..input_vec[indices[i]].len() - 1], 10).unwrap();
            }
        }
    }

    if coordinates[3] == 2000000 {
        beacons_on_line.insert(coordinates[2]);
    }

    let x_distance: usize = coordinates[0].abs_diff(coordinates[2]);
    let y_distance: usize = coordinates[1].abs_diff(coordinates[3]);

    Sensor {
        x: coordinates[0],
        y: coordinates[1],
        distance_to_closest: x_distance + y_distance
    }    
}

pub fn calc_distance(sensor: &Sensor, coordinates: (isize, isize)) -> usize {
    let x_distance: usize = (sensor.x).abs_diff(coordinates.0);
    let y_distance: usize = (sensor.y).abs_diff(coordinates.1);

    x_distance + y_distance
}
