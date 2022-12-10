use std::fs;
use std::env;
use day08_trees::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {

            let mut forest: Forest = grow_forest(input);
            let mut vis_count: usize = 0;

            for i in 0..(forest.len()) {
                crawl(&mut forest, i, Direction::Right, &mut vis_count);
                crawl(&mut forest, i, Direction::Left , &mut vis_count);
                crawl(&mut forest, i, Direction::Up , &mut vis_count);
                crawl(&mut forest, i, Direction::Down , &mut vis_count);
            }

            println!("⭐ First star ⭐ - Visible trees: {}", vis_count);

            // LOOPS!

            let mut top_score: usize = 0;

            for i in 1..(forest.len() - 1) {
                for j in 1..(forest[i].len() - 1) {
                    let tree_score: usize = calculate_score(i, j, &forest);
                    if tree_score > top_score {
                        top_score = tree_score
                    }
                }
            }

            println!("🌟 Second star ✨ - Highest score: {}", top_score);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
