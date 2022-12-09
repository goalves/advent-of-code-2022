use itertools::Itertools;

fn main() {
    let input = include_str!("../../day9_input");
    // let input = include_str!("../../test_inputs/day9_test");

    //Phase 1 would be knot size 2
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
        let len = knots.len();
        let direction_vector = direction_from_str(direction);

        for _i in 0..number_of_moves {
            knots[len - 1].0 += direction_vector.0;
            knots[len - 1].1 += direction_vector.1;

            for index in (0..knots.len() - 1).rev() {
                if let Some(tail_move) =
                    tail_move(&knots[index as usize], &knots[index as usize + 1])
                {
                    knots[index as usize] = tail_move;
                    tail_visits.push(knots[0])
                };
            }
        }
    }

    println!("Result: {}", tail_visits.iter().unique().count());
}

fn tail_move(tail: &(i32, i32), next: &(i32, i32)) -> Option<(i32, i32)> {
    let dx = next.0 - tail.0;
    let dy = next.1 - tail.1;

    if dx.abs() < 2 && dy.abs() < 2 {
        return None;
    }

    Some((tail.0 + dx.signum(), tail.1 + dy.signum()))
}

fn direction_from_str(direction: &str) -> (i32, i32) {
    match direction {
        "D" => (0, -1),
        "U" => (0, 1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => panic!("no mommy no"),
    }
}
