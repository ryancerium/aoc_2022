fn part_one(mut forest: Vec<Vec<(i32, bool)>>) {
    let h = forest.len();
    let w = forest[0].len();

    for row in 0..h {
        let mut max_height = -1;
        for col in 0..w {
            if forest[row][col].0 > max_height {
                forest[row][col].1 = true;
                max_height = forest[row][col].0;
            }
        }

        let mut max_height = -1;
        for col in (0..w).rev() {
            if forest[row][col].0 > max_height {
                forest[row][col].1 = true;
                max_height = forest[row][col].0;
            }
        }
    }

    for col in 0..w {
        let mut max_height = -1;
        for row in 0..h {
            if forest[row][col].0 > max_height {
                forest[row][col].1 = true;
                max_height = forest[row][col].0;
            }
        }

        let mut max_height = -1;
        for row in (0..h).rev() {
            if forest[row][col].0 > max_height {
                forest[row][col].1 = true;
                max_height = forest[row][col].0;
            }
        }
    }

    // println!("\n\n");

    let mut visible = 0;
    for grove in forest.iter() {
        print!("  ");
        for tree in grove.iter() {
            if tree.1 {
                visible += 1;
            }
            print!("{}", if tree.1 { 'X' } else { '.' });
        }
        print!("\n");
    }
    println!("\n{} visible trees\n\n-------------------", visible);
}

fn get_scenic_score(
    mut forest: Vec<Vec<(i32, bool)>>,
    tree_row: usize,
    tree_col: usize,
    print: bool,
) -> usize {
    let h = forest.len();
    let w = forest[0].len();

    let tree_height = forest[tree_row][tree_col].0;

    let mut visible_left = 0;
    for (i, c) in (0..tree_col).rev().enumerate() {
        visible_left = i + 1;
        forest[tree_row][c].1 = true;
        if forest[tree_row][c].0 >= tree_height {
            break;
        }
    }

    let mut visible_right = 0;
    for (i, c) in ((tree_col + 1)..w).enumerate() {
        visible_right = i + 1;
        forest[tree_row][c].1 = true;
        if forest[tree_row][c].0 >= tree_height {
            break;
        }
    }

    let mut visible_up = 0;
    for (i, r) in (0..tree_row).rev().enumerate() {
        visible_up = i + 1;
        forest[r][tree_col].1 = true;
        if forest[r][tree_col].0 >= tree_height {
            break;
        }
    }

    let mut visible_down = 0;
    for (i, r) in ((tree_row + 1)..h).enumerate() {
        visible_down = i + 1;
        forest[r][tree_col].1 = true;
        if forest[r][tree_col].0 >= tree_height {
            break;
        }
    }

    let score = visible_left * visible_right * visible_up * visible_down;

    if print {
        for (r, grove) in forest.iter().enumerate() {
            print!("  ");
            for (c, tree) in grove.iter().enumerate() {
                if r == tree_row && c == tree_col {
                    print!("{}", tree.0);
                } else if tree.1 {
                    print!("{}", tree.0);
                    //print!("X");
                } else {
                    print!(".");
                    //print!("{}", tree.0);
                }
            }
            print!("\n");
        }

        println!("Scenic score: {}\n", score);
    }

    return score;
}

fn part_two(forest: Vec<Vec<(i32, bool)>>) {
    let h = forest.len();
    let w = forest[0].len();

    let mut position = (0, 0);
    let mut score = 0;
    for row in 0..h {
        for col in 0..w {
            let new_score = get_scenic_score(forest.clone(), row, col, false);
            if new_score > score {
                position = (col, row);
                score = new_score;
            }
        }
    }

    get_scenic_score(forest.clone(), position.1, position.0, true);

    println!(
        "Best score: {} is at row: {} col: {}",
        score, position.0, position.1
    );
}

fn main() -> eyre::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).unwrap_or(&"input.txt".to_owned()).clone();
    let s = std::fs::read_to_string(input)?;

    let mut forest: Vec<Vec<(i32, bool)>> = Vec::new();

    for line in s.lines() {
        let mut grove = Vec::new();
        for c in line.chars() {
            grove.push((c.to_string().parse::<i32>().unwrap(), false));
        }
        forest.push(grove);
    }

    // for grove in forest.iter() {
    //     print!("  ");
    //     for tree in grove.iter() {
    //         print!("{}", tree.0);
    //     }
    //     print!("\n");
    // }

    part_one(forest.clone());
    part_two(forest.clone());

    Ok(())
}
