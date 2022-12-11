use evalexpr::*;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<i64>,
    operation: String,
    test_disivible: u32,
    true_case_throw: usize,
    false_case_throw: usize,
    inspected_items: u32,
}

fn main() {
    let input = include_str!("../../day11_input");
    // let input = include_str!("../../test_inputs/day11_test");
    let mut monkeys: Vec<Monkey> = build_monkeys(input);

    println!("{:?}", monkeys);
    start_game(20, &mut monkeys);

    println!("{:?}", monkeys);

    monkeys.sort_by(|a, b| b.inspected_items.cmp(&a.inspected_items));

    println!(
        "Dman yo monke: {:?}",
        monkeys
            .iter()
            .take(2)
            .map(|x| x.inspected_items)
            .product::<u32>()
    );
}

fn build_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey_lines| {
            let split_lines: Vec<&str> = monkey_lines.lines().collect();

            let items: VecDeque<i64> = split_lines[1]
                .split("Starting items: ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|x| x.parse::<i64>().unwrap())
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

    while round < iterations {
        while let Some(item) = monkeys[turn].items.pop_front() {
            monkeys[turn].inspected_items += 1;
            // apply function on item value
            let func = monkeys[turn].operation.replace("old", &item.to_string());

            // get bored, divide by 3 and round up
            let result = eval_int(&func).unwrap() / 3;
            if result < 0 {
                panic!(
                    "result should never be negative: {}, func {}/3, item was {}",
                    result, func, item
                )
            }

            // push to other monkey
            let mut receiving_monkey_index = monkeys[turn].false_case_throw;

            if result % monkeys[turn].test_disivible as i64 == 0 {
                receiving_monkey_index = monkeys[turn].true_case_throw;
            }

            monkeys[receiving_monkey_index].items.push_back(result);
        }
        turn += 1;
        if turn == monkeys.len() {
            round += 1;
            turn = 0;
        }
    }
}
