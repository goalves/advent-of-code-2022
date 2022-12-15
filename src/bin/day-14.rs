use core::fmt;

fn main() {
    let input = include_str!("../../day14_input");
    // let input = include_str!("../../test_inputs/day14_test");

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

    let mut max_depth: usize = 0;
    for path in rock_paths.iter().flatten() {
        if max_depth < path.y as usize {
            max_depth = path.y as usize;
        }
    }

    let mut matrix = vec![vec![Place::None; max_depth + 3]; 1000];

    for paths in rock_paths.iter() {
        let positions = positions_to_fill_for_paths(paths);

        for (x, y) in positions.into_iter() {
            matrix[x][y] = Place::Rock;
        }
    }

    print_matrix(&matrix, 493, 504, 3, 9);

    let sand_iterations = start_darude_sandstorm(&mut matrix, max_depth);
    println!("Phase 1: {}", sand_iterations);

    // phase 2, clear stuff and run again
    for x in 0..matrix.len() {
        for y in 0..matrix[0].len() {
            if matrix[x][y] == Place::Sand {
                matrix[x][y] = Place::None;
            }
        }
    }

    // setup cave ground
    for x in 0..matrix.len() {
        matrix[x][max_depth + 2] = Place::Rock;
    }

    let sand_iterations = start_darude_sandstorm(&mut matrix, max_depth + 3);
    println!("Phase 2: {}", sand_iterations);
}

fn start_darude_sandstorm(matrix: &mut Vec<Vec<Place>>, max_depth: usize) -> usize {
    let mut sand_iterations = 0;
    loop {
        sand_iterations += 1;
        let mut sand_position: (usize, usize) = (500, 0);
        while let Some(new_position) = calculate_new_sand_position(&sand_position, &matrix) {
            if new_position.1 >= max_depth {
                return sand_iterations - 1;
            }

            sand_position = new_position;
        }

        if sand_position == (500, 0) {
            return sand_iterations;
        }

        matrix[sand_position.0][sand_position.1] = Place::Sand;
    }
}

fn calculate_new_sand_position(
    position: &(usize, usize),
    matrix: &Vec<Vec<Place>>,
) -> Option<(usize, usize)> {
    if matrix[position.0][position.1 + 1] == Place::None {
        return Some((position.0, position.1 + 1));
    }

    if matrix[position.0 - 1][position.1 + 1] == Place::None {
        return Some((position.0 - 1, position.1 + 1));
    }

    if matrix[position.0 + 1][position.1 + 1] == Place::None {
        return Some((position.0 + 1, position.1 + 1));
    }

    return None;
}

fn print_matrix(
    matrix: &Vec<Vec<Place>>,
    x_start: usize,
    x_end: usize,
    y_start: usize,
    y_end: usize,
) {
    for y in y_start..=y_end {
        for x in x_start..=x_end {
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

#[derive(Debug, Clone, PartialEq)]
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
