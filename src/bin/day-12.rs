use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../day12_input");
    // let input = include_str!("../../test_inputs/day12_test");

    let mut starting_point = (0, 0);
    let mut finish_point = (0, 0);
    let mut lowest_points = Vec::new();
    lowest_points.push(starting_point);

    let field: Vec<Vec<u8>> = input
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(|(y, letter)| {
                    if letter == 'S' {
                        starting_point = (x, y);
                        0
                    } else if letter == 'E' {
                        finish_point = (x, y);
                        26
                    } else {
                        if letter == 'a' {
                            lowest_points.push((x, y))
                        }
                        letter as u8 - 96
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
        .into();

    println!(
        "Starting point: {:?}, Finishing point: {:?}",
        starting_point, finish_point
    );

    println!("first phase: {}", dfs(&field, starting_point, finish_point));

    println!(
        "second phase: {}",
        lowest_points
            .into_iter()
            .map(|point| dfs(&field, point, finish_point))
            .min()
            .unwrap()
    );
}

fn dfs(field: &Vec<Vec<u8>>, start: (usize, usize), finish: (usize, usize)) -> usize {
    let mut to_visit = VecDeque::new();
    to_visit.push_back((start, 0));

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some((node, step_count)) = to_visit.pop_front() {
        if visited.contains(&node) {
            continue;
        }

        if node == finish {
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

fn get_neighbors(field: &Vec<Vec<u8>>, node: (usize, usize)) -> Vec<(usize, usize)> {
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

    neighbors
        .into_iter()
        .filter(|&(x, y)| field[x][y] as i32 - field[node.0][node.1] as i32 <= 1)
        .collect()
}
