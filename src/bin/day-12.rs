use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../day12_input");
    // let input = include_str!("../../test_inputs/day12_test");

    let field: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| {
                    if x == 'S' {
                        0
                    } else if x == 'E' {
                        27
                    } else {
                        x as u32 - 96
                    }
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
        .into();

    println!("{}", input);
    println!("{:?}", field);

    let mut starting_point = (0, 0);
    let mut finish_point = (0, 0);

    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if field[i][j] == 0 {
                starting_point = (i, j);
            }
            if field[i][j] == 27 {
                finish_point = (i, j);
            }
        }
    }

    println!(
        "Starting point: {:?}, Finishing point: {:?}",
        starting_point, finish_point
    );

    print_field(&field);

    println!("{}", dfs(&field, starting_point, finish_point));
}

fn dfs(field: &Vec<Vec<u32>>, start: (usize, usize), finish: (usize, usize)) -> usize {
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start, 0));

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((node, step_count)) = to_visit.pop_front() {
        if visited.contains(&node) {
            continue;
        }

        println!("current new node: {:?} on step count: {}", node, step_count);

        if node == finish {
            println!("node{:?}: len of visited {}", node, visited.len());
            return step_count;
        }

        visited.insert(node);

        for neighbor in get_neighbors(&field, node) {
            to_visit.push_back((neighbor, step_count + 1))
        }

        to_visit = to_visit.into_iter().unique().collect();
    }

    return usize::MAX;
}

fn get_neighbors(field: &Vec<Vec<u32>>, node: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let (i, j) = node;

    if i > 0 {
        neighbors.push((i - 1, j))
    }

    if i < field.len() - 1 {
        neighbors.push((i + 1, j))
    }

    if j > 0 {
        neighbors.push((i, j - 1))
    }

    if j < field[0].len() - 1 {
        neighbors.push((i, j + 1))
    }

    // println!("neighbors: {:?}", neighbors);

    neighbors
        .into_iter()
        .filter(|&(x, y)| field[node.0][node.1] as i32 - field[x][y] as i32 >= -1)
        .collect()
}

fn print_field(field: &Vec<Vec<u32>>) {
    for line in field {
        println!("{:?}", line)
    }
}
