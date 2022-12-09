use std::cmp::max;

#[derive(Clone, Debug)]
struct Tree {
    pub heigth: u32,
    pub top: u32,
    pub down: u32,
    pub left: u32,
    pub right: u32,
}

impl Tree {
    fn new(val: u32) -> Self {
        Self {
            heigth: val,
            top: val,
            down: val,
            left: val,
            right: val,
        }
    }
}

// Create a cost matrix to fill from all angles (left -> right, right -> left, top -> down, down -> top)
// This matrix has the summed up cost heights to reach every point from every direction, later on we can
// compare the end result just by looking at one position to the direction we want.
fn fill_cost_heigths(data: &mut Vec<Vec<Tree>>) {
    let row_len = data.len();
    let col_len = data[0].len();

    for row in 0..row_len {
        for col in 0..col_len {
            if col > 0 {
                data[row][col].left = max(data[row][col].left, data[row][col - 1].left);
            }
        }

        for col in (0..col_len).rev() {
            if col < col_len - 1 {
                data[row][col].right = max(data[row][col].right, data[row][col + 1].right);
            }
        }
    }

    for col in 0..col_len {
        for row in 0..row_len {
            if row > 0 {
                data[row][col].top = max(data[row][col].top, data[row - 1][col].top);
            }
        }

        for row in (0..row_len).rev() {
            if row < row_len - 1 {
                data[row][col].down = max(data[row][col].down, data[row + 1][col].down);
            }
        }
    }
}

fn total_visible_trees(data: &Vec<Vec<Tree>>) -> u32 {
    let mut total_trees = 0;

    let row_len = data.len();
    let col_len = data[0].len();
    for row in 0..row_len {
        for col in 0..col_len {
            if row == 0 || col == 0 || row == row_len - 1 || col == col_len - 1 {
                total_trees += 1;
            } else {
                let top = data[row][col].heigth > data[row - 1][col].top;
                let bottom = data[row][col].heigth > data[row + 1][col].down;
                let left = data[row][col].heigth > data[row][col - 1].left;
                let right = data[row][col].heigth > data[row][col + 1].right;
                if top || bottom || left || right {
                    total_trees += 1;
                }
            }
        }
    }

    total_trees
}

fn tree_scenic_score(
    row: usize,
    col: usize,
    row_step: i32,
    col_step: i32,
    data: &Vec<Vec<Tree>>,
) -> u32 {
    let height = data[row][col].heigth;
    let mut x = row as i32 + row_step;
    let mut y = col as i32 + col_step;
    let mut score = 0;

    while x >= 0 && x < data.len() as i32 && y >= 0 && y < data[0].len() as i32 {
        score += 1;

        if data[x as usize][y as usize].heigth >= height {
            break;
        }

        x += row_step;
        y += col_step;
    }

    score
}

fn calculate_scenic_score(data: &mut Vec<Vec<Tree>>) -> u32 {
    let row_len = data.len();
    let col_len = data[0].len();
    let mut max_score = 1;

    for row in 0..row_len {
        for col in 0..col_len {
            if row != 0 && col != 0 && row != row_len - 1 && col != col_len - 1 {
                let scores = vec![
                    tree_scenic_score(row, col, -1, 0, data),
                    tree_scenic_score(row, col, 1, 0, data),
                    tree_scenic_score(row, col, 0, -1, data),
                    tree_scenic_score(row, col, 0, 1, data),
                ];

                max_score = max(max_score, scores.iter().product());
            }
        }
    }

    max_score
}

fn main() {
    let mut data = vec![];

    let input = include_str!("../../day8_input").lines();

    for line in input {
        let mut curr_vec = vec![];
        for char in line.chars() {
            let val = char.to_string().parse::<u32>().unwrap();
            curr_vec.push(Tree::new(val));
        }
        data.push(curr_vec.clone());
    }

    fill_cost_heigths(&mut data);
    let total_trees = total_visible_trees(&data);
    println!("{total_trees}");

    let total_scenic_score = calculate_scenic_score(&mut data);
    println!("{total_scenic_score}");
}
