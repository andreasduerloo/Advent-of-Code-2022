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

            let mut score_vec: Vec<usize> = Vec::new();

            for i in 0..99 {
                for j in 0..99 {
                    let mut left_score: usize = 0;
                    if j != 0 {
                        for k in 1..(j + 1) {
                            if forest[i][j - k].height < forest[i][j].height {
                                left_score += 1;
                            } else {
                                left_score += 1;
                                break
                            }
                        }
                    }
                    

                    let mut top_score: usize = 0;
                    if i != 0 {
                        for k in 1..(i + 1) {
                            if forest[i - k][j].height < forest[i][j].height {
                                top_score += 1;
                            } else {
                                top_score += 1;
                                break
                            }
                        }
                    }
                    

                    let mut right_score: usize = 0;
                    if j != 98 {
                        for k in (j + 1)..99 {
                            if forest[i][k].height < forest[i][j].height {
                                right_score += 1;
                            } else {
                                right_score += 1;
                                break
                            }
                        }
                    }
                    

                    let mut down_score: usize = 0;
                    if i != 98 {
                        for k in (i + 1)..99 {
                            if forest[k][j].height < forest[i][j].height {
                                down_score += 1;
                            } else {
                                down_score += 1;
                                break
                            }
                        }
                    }

                    score_vec.push(left_score * top_score * right_score * down_score);
                }
            }

            score_vec.sort();
            score_vec.reverse();

            println!("🌟 Second star ✨ - Highest score: {}", score_vec[0]);

        } else {
            eprintln!("Could not read file. Exiting. 🦌");
        }
    } else {
        eprintln!("No argument was passed. Exiting. 🦌");
    }
}
