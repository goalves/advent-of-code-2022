use itertools::Itertools;

fn main() {
    // let input = include_str!("../../day9_input");
    let input = include_str!("../../test_inputs/day9_test");

    //Phase 1
    let knot_size = 10;

    let mut knots = Vec::new();
    for _ in 0..knot_size {
        knots.push((0i32, 0i32));
    }

    let mut tail_visits = vec![(0i32, 0i32)];

    for line in input.lines() {
        let split: Vec<&str> = line.split(' ').collect();
        let direction = split[0];
        let number_of_moves = split[1].parse::<u32>().unwrap();

        for _i in 0..number_of_moves {
            let head = knots.last().unwrap();

            match direction {
                "D" => {
                    let new_position = (head.0, head.1 + 1);
                    let new_tail =
                        update_positions(new_position, (head.0, head.1), &mut knots, knot_size - 1);
                    tail_visits.push(new_tail);
                }
                "U" => {
                    let new_position = (head.0, head.1 - 1);
                    let new_tail =
                        update_positions(new_position, (head.0, head.1), &mut knots, knot_size - 1);
                    tail_visits.push(new_tail);
                }
                "L" => {
                    let new_position = (head.0 - 1, head.1);
                    let new_tail =
                        update_positions(new_position, (head.0, head.1), &mut knots, knot_size - 1);
                    tail_visits.push(new_tail);
                }
                "R" => {
                    let new_position = (head.0 + 1, head.1);
                    let new_tail =
                        update_positions(new_position, (head.0, head.1), &mut knots, knot_size - 1);
                    tail_visits.push(new_tail);
                }
                _ => panic!("no mommy no"),
            }
        }

        println!(
            "total tail after running move {:?}: {:?}",
            split, tail_visits
        );
    }

    println!("{}", tail_visits.iter().unique().count());
}

fn update_positions(
    new_point: (i32, i32),
    current_value: (i32, i32),
    knots: &mut Vec<(i32, i32)>,
    current_index: i32,
) -> (i32, i32) {
    println!("called update positions");
    knots[current_index as usize] = (new_point.0, new_point.1);
    let previous_index = current_index - 1;

    if previous_index >= 0 && should_move_knot(&new_point, &knots[previous_index as usize]) {
        let previous_value = knots[previous_index as usize].clone();
        knots[previous_index as usize] = current_value;
        println!("Updated knots because had to move: {:?}", knots);
        update_positions(
            knots[previous_index as usize],
            previous_value,
            knots,
            previous_index,
        );
    }

    knots[0]
}

fn should_move_knot(this: &(i32, i32), previous: &(i32, i32)) -> bool {
    println!("comparing: {:?} to prev: {:?}", this, previous);
    !((previous.0 == this.0 || previous.0 == (this.0 - 1) || previous.0 == (this.0 + 1))
        && (previous.1 == this.1 || previous.1 == (this.1 - 1) || previous.1 == (this.1 + 1)))
}
