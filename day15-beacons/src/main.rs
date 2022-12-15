use std::fs;
use std::env;
use std::collections::HashSet;
use day15_beacons::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {

            let mut sensors: Vec<Sensor> = Vec::new();
            let mut beacons_on_line: HashSet<isize> = HashSet::new();

            for line in input.lines() {
                sensors.push(create_sensor(line, &mut beacons_on_line));
            }

            let mut impossible: usize = 0;

            for i in -1000000..6000000 {
                for j in 0..sensors.len() {
                    if calc_distance(&sensors[j], (i, 2000000)) <= sensors[j].distance_to_closest {
                        impossible += 1;
                        break
                    }
                }
            }

            println!("{} spots are impossible", impossible - beacons_on_line.len());

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
