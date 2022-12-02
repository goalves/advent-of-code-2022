fn main() {
    let mut results: Vec<u64> = include_str!("../../day2_input")
        .split("\n\n")
        .into_iter()
        .map(|calories_str| {
            calories_str
                .split("\n")
                .map(|x| x.parse::<u64>().unwrap_or(0))
                .sum::<u64>()
        })
        .collect();

    results.sort_unstable_by(|a, b| b.cmp(a));

    println!("{}", results.iter().take(3).sum::<u64>());
}
