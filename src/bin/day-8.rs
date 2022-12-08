use std::{
    cmp::{max, min},
    fmt::Display,
};

#[derive(Clone, Debug)]
struct Tree {
    pub value: u32,
    pub top_cost: u32,
    pub bottom_cost: u32,
    pub left_cost: u32,
    pub right_cost: u32,
}

impl Tree {
    fn new(val: u32) -> Self {
        Self {
            value: val,
            top_cost: val,
            bottom_cost: val,
            left_cost: val,
            right_cost: val,
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(V:{}, T: {}, B: {}, L: {}, R: {})",
            self.value, self.top_cost, self.bottom_cost, self.left_cost, self.right_cost
        )
    }
}

fn fill_cost_values(data: &mut Vec<Vec<Tree>>) {
    let row_len = data.len();
    let col_len = data[0].len();

    for row in 0..row_len {
        for col in 0..col_len {
            if col > 0 {
                data[row][col].left_cost =
                    max(data[row][col].left_cost, data[row][col - 1].left_cost);
            }
        }

        for col in (0..col_len).rev() {
            if col < col_len - 1 {
                data[row][col].right_cost =
                    max(data[row][col].right_cost, data[row][col + 1].right_cost);
            }
        }
    }

    for col in 0..col_len {
        for row in 0..row_len {
            if row > 0 {
                data[row][col].top_cost = max(data[row][col].top_cost, data[row - 1][col].top_cost);
            }
        }

        for row in (0..row_len).rev() {
            if row < row_len - 1 {
                data[row][col].bottom_cost =
                    max(data[row][col].bottom_cost, data[row + 1][col].bottom_cost);
            }
        }
    }
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

    let mut total_trees = 0;
    fill_cost_values(&mut data);

    let row_len = data.len();
    let col_len = data[0].len();
    for row in 0..row_len {
        for col in 0..col_len {
            if row == 0 || col == 0 || row == row_len - 1 || col == col_len - 1 {
                total_trees += 1;
            } else {
                let top = data[row][col].value > data[row - 1][col].top_cost;
                let bottom = data[row][col].value > data[row + 1][col].bottom_cost;
                let left = data[row][col].value > data[row][col - 1].left_cost;
                let right = data[row][col].value > data[row][col + 1].right_cost;
                if top || bottom || left || right {
                    println!("visible!: {:?}", data[row][col]);
                    total_trees += 1;
                }
            }
        }
    }

    for line in data {
        for el in line {
            print!("{} ", el);
        }
        println!("\n")
    }

    println!("{}", total_trees);
}
