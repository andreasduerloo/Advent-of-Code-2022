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

pub fn calculate_score(row: usize, column: usize, forest: &Forest) -> usize {
    let mut scores: [usize; 4] = [0; 4];
    let dimension: usize = forest.len();

    // Look to the right
    for i in (column + 1)..dimension {
        if forest[row][i].height < forest[row][column].height {
            scores[0] += 1;
        } else {
            scores[0] += 1;
            break
        }
    }

    // Look to the left
    for i in 1..(column + 1) {
        if forest[row][column - i].height < forest[row][column].height {
            scores[1] += 1;
        } else {
            scores[1] += 1;
            break
        }
    }

    // Look down
    for i in (row + 1)..dimension {
        if forest[i][column].height < forest[row][column].height {
            scores[2] += 1;
        } else {
            scores[2] += 1;
            break
        }
    }

    // Look up
    for i in 1..(row + 1) {
        if forest[row - i][column].height < forest[row][column].height {
            scores[3] += 1;
        } else {
            scores[3] += 1;
            break
        }
    }

    scores.into_iter().reduce(|acc, s| { acc * s }).unwrap()
}