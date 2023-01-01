fn main() {
    let part_1_result = include_str!("../../inputs/day02")
        .lines()
        .map(|line| match line {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2,
            "C Z" => 3 + 3,
            _ => panic!("heh dumb you"),
        })
        .sum::<u64>();

    println!("{part_1_result}");

    let part_two_result = include_str!("../../inputs/day02")
        .lines()
        .map(|line| match line {
            "A X" => 3,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => panic!("heh dumb you"),
        })
        .sum::<u64>();

    println!("{part_two_result}");
}
