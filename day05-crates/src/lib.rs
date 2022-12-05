pub struct Config<'a> {
    pub stack1: Vec<Box<Vec<char>>>,
    pub stack2: Vec<Box<Vec<char>>>,
    pub instructions: Vec<&'a str>
}

impl Config<'_> {
    pub fn new(stack1: Vec<Box<Vec<char>>>, stack2: Vec<Box<Vec<char>>>, instructions: Vec<&str>) -> Config {
        Config {
            stack1,
            stack2,
            instructions
        }
    }
}

pub fn setup(mut input_vec: Vec<&str>) -> Config {
    // Find the empty line - the line above is a header, the line below is the first instruction

    let mut emptyline: usize = 0;

    for i in 0..input_vec.len() {
        if input_vec[i] == "" {
            emptyline = i;
            break
        }
    }

    // Find the number of 'columns' based on the bottom row

    let stack_count: usize = (input_vec[emptyline - 1].len() + 1) / 4;

    // Create a vector with pointers to vectors to represent the stacks

    let mut stacks1: Vec<Box<Vec<char>>> = Vec::new();

    for _i in 0..stack_count {
        stacks1.push(Box::new(Vec::new()));
    }

    // Populate the stacks based on the input

    for i in 0..(emptyline - 1) {
        let line_chars: Vec<char> = input_vec[emptyline - (2 + i)].chars().collect();
        for j in 0..stack_count {
            if line_chars[1 + 4 * j] != ' ' {
                stacks1[j].push(line_chars[1 + 4 * j]);
            }
        }
    }

    let stacks2 = stacks1.clone();

    // Grab the rest of the input, those are the instructions

    let instructions: Vec<&str> = input_vec.drain((emptyline + 1)..).collect();

    Config::new(stacks1, stacks2, instructions)
}

pub fn apply_9000(instruction: &str, stacks: &mut Vec<Box<Vec<char>>>) {
    let instruction_vec: Vec<&str> = instruction.split_whitespace().collect();

    let count: usize = usize::from_str_radix(instruction_vec[1], 10).unwrap();
    let from: usize = usize::from_str_radix(instruction_vec[3], 10).unwrap() - 1;
    let to: usize = usize::from_str_radix(instruction_vec[5], 10).unwrap() - 1;

    for _i in 0..count {
        let crate_to_move: char = stacks[from].pop().unwrap();
        stacks[to].push(crate_to_move);
    }
}

pub fn apply_9001(instruction: &str, stacks: &mut Vec<Box<Vec<char>>>) {
    let instruction_vec: Vec<&str> = instruction.split_whitespace().collect();

    let from: usize = usize::from_str_radix(instruction_vec[3], 10).unwrap() - 1;
    let index: usize = stacks[from].len() - usize::from_str_radix(instruction_vec[1], 10).unwrap();
    let to: usize = usize::from_str_radix(instruction_vec[5], 10).unwrap() - 1;

    let mut crates_to_move: Vec<char> = stacks[from].drain(index..).collect();
    stacks[to].append(&mut crates_to_move);
}