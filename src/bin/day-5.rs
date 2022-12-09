use std::{collections::VecDeque, str::Lines};

#[derive(Debug, Default)]
struct Stack {
    crates: VecDeque<char>,
}

impl Stack {
    pub fn add_crate(&mut self, crate_data: char) {
        self.crates.push_back(crate_data)
    }
}

#[derive(Debug)]
struct Move {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let split_values = value.split(' ').collect::<Vec<&str>>();

        Self {
            count: split_values[1].parse::<usize>().unwrap(),
            from: split_values[3].parse::<usize>().unwrap(),
            to: split_values[5].parse::<usize>().unwrap(),
        }
    }
}

fn build_moves_from_input(input: Lines) -> Vec<Move> {
    let mut moves = Vec::new();

    for line in input {
        moves.push(Move::from(line));
    }

    moves
}

fn apply_moves(stacks: &mut [Stack], moves: Vec<Move>) {
    for action in moves {
        for _ in 0..action.count {
            let value = &mut stacks[action.from - 1].crates.pop_front().unwrap();
            stacks[action.to - 1].crates.push_front(*value);
        }
    }
}

fn apply_moves_crate_mover_9001(stacks: &mut [Stack], moves: Vec<Move>) {
    for action in moves {
        let mut values = Vec::new();
        for _ in 0..action.count {
            let value = &mut stacks[action.from - 1].crates.pop_front().unwrap();
            values.push(*value);
        }

        values.reverse();
        for value in values {
            stacks[action.to - 1].crates.push_front(value);
        }
    }
}

fn stack_tops(stacks: &mut Vec<Stack>) -> Vec<char> {
    let mut stack_tops = Vec::new();
    for stack in stacks {
        let value = stack.crates.pop_front().unwrap_or(' ');
        stack_tops.push(value)
    }

    stack_tops
}

fn main() {
    let mut data = include_str!("../../day5_input").split("\n\n");
    let crate_lines = data.next().unwrap().lines();
    let move_lines = data.next().unwrap().lines();

    let mut stacks = build_stacks_from_input(crate_lines);
    let moves = build_moves_from_input(move_lines);

    apply_moves(&mut stacks, moves);

    let tops = stack_tops(&mut stacks);
    println!("Normal Moves: {}", tops.into_iter().collect::<String>());

    let mut data = include_str!("../../day5_input").split("\n\n");
    let crate_lines = data.next().unwrap().lines();
    let move_lines = data.next().unwrap().lines();

    let mut stacks = build_stacks_from_input(crate_lines);
    let moves = build_moves_from_input(move_lines);

    apply_moves_crate_mover_9001(&mut stacks, moves);

    let tops = stack_tops(&mut stacks);
    println!(
        "Moves Crate Moover 9001: {}",
        tops.into_iter().collect::<String>()
    );
}

fn build_stacks_from_input(mut input: Lines) -> Vec<Stack> {
    let last_line = input.next_back().unwrap();
    let number_of_stacks = last_line.len() / 4;

    let mut stacks: Vec<Stack> = Vec::with_capacity(number_of_stacks);
    for _ in 0..number_of_stacks + 1 {
        stacks.push(Stack::default())
    }

    for line in input {
        let mut index = 0;
        for (line_index, char) in line.chars().enumerate() {
            if line_index == 1 || (line_index % 4 == 1 && line_index != 0) {
                if char != ' ' {
                    stacks[index].add_crate(char);
                }
                index += 1;
            }
        }
    }

    stacks
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_phase_1() {
        let mut test_data = include_str!("../../test_inputs/day5_test").split("\n\n");
        let crate_lines = test_data.next().unwrap().lines();
        let move_lines = test_data.next().unwrap().lines();

        let mut stacks = build_stacks_from_input(crate_lines);
        let moves = build_moves_from_input(move_lines);

        apply_moves(&mut stacks, moves);

        let tops = stack_tops(&mut stacks);
        assert_eq!(tops, vec!['C', 'M', 'Z']);
    }

    #[test]
    fn test_phase_2() {
        let mut test_data = include_str!("../../test_inputs/day5_test").split("\n\n");
        let crate_lines = test_data.next().unwrap().lines();
        let move_lines = test_data.next().unwrap().lines();

        let mut stacks = build_stacks_from_input(crate_lines);
        let moves = build_moves_from_input(move_lines);

        apply_moves_crate_mover_9001(&mut stacks, moves);

        let tops = stack_tops(&mut stacks);
        assert_eq!(tops, vec!['M', 'C', 'D']);
    }
}
