use evalexpr::*;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: String,
    test_disivible: u32,
    true_case_throw: usize,
    false_case_throw: usize,
    inspected_items: u64,
}

fn main() {
    let input = include_str!("../../inputs/day11");
    // let input = include_str!("../../inputs/test/day11");
    let mut monkeys: Vec<Monkey> = build_monkeys(input);

    println!("{:?}", monkeys);
    start_game(10000, &mut monkeys);

    println!("{:?}", monkeys);

    monkeys.sort_by(|a, b| b.inspected_items.cmp(&a.inspected_items));

    println!(
        "Dman yo monke: {:?}",
        monkeys
            .iter()
            .take(2)
            .map(|x| x.inspected_items)
            .product::<u64>()
    );
}

fn build_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey_lines| {
            let split_lines: Vec<&str> = monkey_lines.lines().collect();

            let items: VecDeque<u64> = split_lines[1]
                .split("Starting items: ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect();

            let operation = split_lines[2].split("= ").nth(1).unwrap().into();
            let test_disivible = split_lines[3]
                .split("Test: divisible by ")
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();

            let true_case_throw = split_lines[4]
                .split("monkey ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            let false_case_throw = split_lines[5]
                .split("monkey ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();

            Monkey {
                operation,
                test_disivible,
                items,
                true_case_throw,
                false_case_throw,
                inspected_items: 0,
            }
        })
        .collect()
}

fn start_game(iterations: usize, monkeys: &mut Vec<Monkey>) {
    let mut round = 0;
    let mut turn = 0;
    let modulus = monkeys
        .iter()
        .map(|x| x.test_disivible as i64)
        .product::<i64>();

    while round < iterations {
        while let Some(item) = monkeys[turn].items.pop_front() {
            monkeys[turn].inspected_items += 1;
            let func = monkeys[turn].operation.replace("old", &item.to_string());
            let result = eval_int(&func).unwrap();
            let mut recv_index = monkeys[turn].false_case_throw;

            if result % monkeys[turn].test_disivible as i64 == 0 {
                recv_index = monkeys[turn].true_case_throw;
            }

            monkeys[recv_index]
                .items
                .push_back((result % modulus) as u64);
        }
        turn += 1;
        if turn == monkeys.len() {
            round += 1;
            turn = 0;
        }
    }
}
