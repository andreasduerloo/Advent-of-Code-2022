#[derive (Clone, Copy)]
pub struct Tree {
    pub height: u8,
    pub visible: bool
}

pub type Forest = [[Tree; 99]; 99];

pub fn grow_forest(input: String) -> Forest {
    let input_vec: Vec<&str> = input.lines().collect();
    let mut forest: Forest = [[Tree { height: 0, visible: false }; 99]; 99];

    for i in 0..input_vec.len() {
        input_vec[i].split("") // Starts and ends with an empty string
            .filter(|s| *s != "")
            .enumerate()
            .for_each(|(j, val)| {
                forest[i][j].height = u8::from_str_radix(val, 10).unwrap();
            });
    }
    forest
}