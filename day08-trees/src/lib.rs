#[derive (Clone, Copy)]
pub struct Tree {
    pub height: u8,
    pub visible: bool
}

pub enum Direction {
    Right,
    Left,
    Up,
    Down,
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

pub fn crawl(forest: &mut Forest, index: usize, direction: Direction, count: &mut usize) {
    let mut tallest: u8 = 0;
    let dimension: usize = forest[index].len();

    for i in 0..dimension {
        let (row, column): (usize, usize) = match direction {
            Direction::Right => {
                (index, i)
            },
            Direction::Left => {
                (index, dimension - (i + 1))
            },
            Direction::Up => {
                (i, index)
            },
            Direction::Down => {
                (dimension - (i + 1), index)
            }
        };

        if forest[row][column].height > tallest || row == 0 || column == 0 || row == (dimension - 1) || column == (dimension - 1) {
            if !forest[row][column].visible {
                forest[row][column].visible = true;
                *count += 1;
            }
            tallest = forest[row][column].height;
            if tallest == 9 {
                return
            }
        }
    }    
}