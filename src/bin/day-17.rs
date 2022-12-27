use std::{collections::HashMap, vec};

#[derive(PartialEq, PartialOrd, Debug)]
enum Jet {
    Left,
    Right,
    Down,
}

#[derive(PartialEq, PartialOrd, Debug)]

enum Rock {
    Horizontal,
    Plus,
    L,
    Vertical,
    Block,
}

impl Rock {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Rock::Horizontal,
            1 => Rock::Plus,
            2 => Rock::L,
            3 => Rock::Vertical,
            4 => Rock::Block,
            _ => panic!("shouldn't pass anything more than 3"),
        }
    }

    pub fn space(&self, position: &(i64, i64)) -> Vec<(i64, i64)> {
        match self {
            Rock::Horizontal => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Rock::L => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Rock::Vertical => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Rock::Block => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
        .into_iter()
        .map(|(x, y)| (x + position.0, y + position.1))
        .collect::<Vec<(i64, i64)>>()
    }
}

fn parse_jets(input: &str) -> Vec<Jet> {
    input
        .trim()
        .chars()
        .into_iter()
        .map(|x| match x {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("invalid char"),
        })
        .collect::<Vec<Jet>>()
}

fn ceiling_map(list: &[i64; 7]) -> Vec<i64> {
    let max = list.iter().max().unwrap() + 1;

    list.clone()
        .into_iter()
        .map(|x| max - x)
        .collect::<Vec<i64>>()
}

// This has one issue: I only store the top of the landing height, which can lead to issues if there is any scneario
// that a rock might have an upper bound but be pushed to a valid platform UNDER the top position
fn fall_rocks(jets: &Vec<Jet>, target: usize) -> i64 {
    let mut landing_height = [0i64; 7];
    let mut jet_index = 0;
    let mut cache: HashMap<(Vec<i64>, usize, usize), (i64, i64)> = HashMap::new();
    let mut height = 0;

    for rock_index in 0..target {
        fall_single(rock_index, &mut landing_height, jets, &mut jet_index);

        let ceiling = ceiling_map(&landing_height);
        height = *landing_height.iter().max().unwrap();
        let state = (ceiling, rock_index % 5, jet_index % jets.len() - 1);

        if cache.contains_key(&state) {
            let (rocks_loop, height_loop) = cache.get(&state).copied().unwrap();
            let rocks_diff = rock_index + 1 - rocks_loop as usize;
            let highest_diff = height - height_loop;
            let repeats = (target - (rock_index + 1)) / rocks_diff;
            let remaining_blocks = (target - (rock_index + 1)) - (repeats * rocks_diff);
            let mut total_height = highest_diff as usize * repeats;
            let mut remaining_rock_index = rock_index + 1;

            (0..remaining_blocks).for_each(|_| {
                fall_single(
                    remaining_rock_index,
                    &mut landing_height,
                    jets,
                    &mut jet_index,
                );
                height = *landing_height.iter().max().unwrap();
                remaining_rock_index += 1;
            });

            total_height += height as usize;
            return total_height as i64;
        } else {
            cache.insert(state, (rock_index as i64 + 1, height));
        }
    }

    height
}

fn fall_single(
    rock_index: usize,
    landing_height: &mut [i64; 7],
    jets: &Vec<Jet>,
    jet_index: &mut usize,
) {
    let rock = Rock::from_index(rock_index % 5);
    let mut rock_position = (2, *landing_height.iter().max().unwrap() + 4);

    loop {
        // move with right/left jets
        let jet = jets.get(*jet_index % jets.len()).unwrap();
        move_rock(&rock, &mut rock_position, &landing_height, jet);
        *jet_index += 1;

        // make the rock fall down with a "down jet rofl"
        if !move_rock(&rock, &mut rock_position, &landing_height, &Jet::Down) {
            break;
        }
    }

    update_highest_landing(&rock, &rock_position, landing_height);
}

fn move_rock(
    rock: &Rock,
    position: &mut (i64, i64),
    landing_height: &[i64; 7],
    movement: &Jet,
) -> bool {
    let new_position = rock_collides(&rock, position, movement, landing_height);
    let moved = new_position != *position;
    *position = new_position;

    moved
}

fn rock_collides(
    rock: &Rock,
    position: &(i64, i64),
    jet: &Jet,
    landing_height: &[i64; 7],
) -> (i64, i64) {
    let new_position = match jet {
        Jet::Left => (position.0.clone() - 1, position.1),
        Jet::Right => (position.0.clone() + 1, position.1),
        Jet::Down => (position.0, position.1.clone() - 1),
    };

    for (x, y) in rock.space(&new_position) {
        if x < 0 || x > 6 || y <= landing_height[x as usize] {
            return *position;
        }
    }

    new_position
}

fn update_highest_landing(rock: &Rock, new_position: &(i64, i64), landing_height: &mut [i64; 7]) {
    let space = rock.space(new_position);

    for (x, y) in space {
        landing_height[x as usize] = landing_height[x as usize].max(y);
    }
}

fn main() {
    let input = include_str!("../../day17_input").trim();
    // let input = include_str!("../../test_inputs/day17_test").trim();
    let jets: Vec<Jet> = parse_jets(input);
    // let part_1 = fall_rocks(&jets, 2022);
    // println!("Max height: {:?}", part_1);

    let part_2 = fall_rocks(&jets, 1000000000000);
    println!("Max height: {:?}", part_2);
}
