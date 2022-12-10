use std::fs;
use std::env;
use day08_trees::*;

fn main() {
    let mut arguments = env::args();

    if let Some(filename) = arguments.nth(1) {
        if let Ok(input) = fs::read_to_string(filename) {

            let mut forest: Forest = grow_forest(input);
            let mut vis_count: usize = 0;

            // Left-to-right
            for i in 0..99 {
                let mut tallest: u8 = 0;
                for j in 0..99 {
                    if forest[i][j].height > tallest || i == 0 || j == 0 || i == 98 || j == 98 {
                        forest[i][j].visible = true;
                        vis_count += 1;
                        tallest = forest[i][j].height;
                        if tallest == 9 {
                            break
                        }
                    }
                }
            }

            // Right-to-left
            for i in 0..99 {
                let mut tallest: u8 = 0;
                for j in 0..99 {
                    if forest[i][98 - j].height > tallest || i == 0 || j == 0 || i == 98 || j == 98 {
                        if !forest[i][98 - j].visible {
                            forest[i][98 - j].visible = true;
                            vis_count += 1;
                        }
                        tallest = forest[i][98 - j].height;
                        if tallest == 9 {
                            break
                        }
                    }
                }
            }

            // Top-to-bottom
            for i in 0..99 {
                let mut tallest: u8 = 0;
                for j in 0..99 {
                    if forest[j][i].height > tallest || i == 0 || j == 0 || i == 98 || j == 98 {
                        if !forest[j][i].visible {
                            forest[j][i].visible = true;
                            vis_count += 1;
                        }
                        tallest = forest[j][i].height;
                        if tallest == 9 {
                            break
                        }
                    }
                }
            }

            // Bottom-to-top
            for i in 0..99 {
                let mut tallest: u8 = 0;
                for j in 0..99 {
                    if forest[98 - j][i].height > tallest || i == 0 || j == 0 || i == 98 || j == 98 {
                        if !forest[98 - j][i].visible {
                            forest[98 - j][i].visible = true;
                            vis_count += 1;
                        }
                        tallest = forest[98 - j][i].height;
                        if tallest == 9 {
                            break
                        }
                    }
                }
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
