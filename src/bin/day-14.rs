use core::fmt;
use std::ops::Range;

fn main() {
    // let input = include_str!("../../day14_input");
    let input = include_str!("../../test_inputs/day14_test");

    let rock_paths: Vec<Vec<RockPath>> = input
        .trim()
        .split("\n")
        .into_iter()
        .map(|lines| {
            lines
                .split(" -> ")
                .map(RockPath::from)
                .collect::<Vec<RockPath>>()
        })
        .collect();

    let mut max_y: usize = 0;
    for path in rock_paths.iter().flatten() {
        if max_y < path.y as usize {
            max_y = path.y as usize;
        }
    }

    let mut matrix = vec![vec![Place::None; 1000]; max_y + 1];

    for paths in rock_paths.iter() {
        let positions = positions_to_fill_for_paths(paths);

        for (x, y) in positions {
            matrix[y][x] = Place::Rock;
        }
    }

    print_matrix(&matrix, 0, 9, 493, 504)
}

fn print_matrix(
    matrix: &Vec<Vec<Place>>,
    x_start: usize,
    x_end: usize,
    y_start: usize,
    y_end: usize,
) {
    for x in x_start..=x_end {
        for y in y_start..=y_end {
            print!("{}", matrix[x][y])
        }
        println!()
    }
}

fn positions_to_fill_for_paths(paths: &Vec<RockPath>) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for index in 0..paths.len() {
        if index < paths.len() - 1 {
            positions.push(points_between(&paths[index], &paths[index + 1]))
        }
    }

    positions.into_iter().flatten().collect()
}

fn points_between(first: &RockPath, second: &RockPath) -> Vec<(usize, usize)> {
    let mut points = vec![];

    let (lower_x, higher_x) = if first.x < second.x {
        (first.x, second.x)
    } else {
        (second.x, first.x)
    };
    let (lower_y, higher_y) = if first.y < second.y {
        (first.y, second.y)
    } else {
        (second.y, first.y)
    };

    for x in lower_x..=higher_x {
        for y in lower_y..=higher_y {
            points.push((x, y))
        }
    }

    points
}

#[derive(Debug)]
struct RockPath {
    x: usize,
    y: usize,
}

impl From<&str> for RockPath {
    fn from(value: &str) -> Self {
        let values = value.split(',').collect::<Vec<&str>>();

        Self {
            x: values[0].parse().unwrap(),
            y: values[1].parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
enum Place {
    None,
    Rock,
    Sand,
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Place::None => write!(f, "  "),
            Place::Rock => write!(f, "🪨"),
            Place::Sand => write!(f, "🥔"),
        }
    }
}
