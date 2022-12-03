#[derive(Debug)]
struct Knapsack<'a> {
    first_half: &'a str,
    second_half: &'a str,
}

impl<'a> From<&'a str> for Knapsack<'a> {
    fn from(value: &'a str) -> Self {
        let (first_half, second_half) = value.split_at(value.len() / 2);

        Knapsack {
            first_half,
            second_half,
        }
    }
}

impl<'a> Knapsack<'a> {
    pub fn find_repeated(&'a self) -> char {
        for char in self.first_half.chars() {
            if self.second_half.contains(char) {
                return char;
            }
        }

        panic!("all knapsacks need to have a repeated item")
    }
}

#[derive(Debug)]
struct Priority(u32);

impl From<char> for Priority {
    fn from(value: char) -> Self {
        let mut digit = value as u32;
        if digit >= 97 {
            digit = digit - 96;
        } else {
            digit = digit - 38;
        }

        Priority(digit)
    }
}

fn main() {
    let input_data = include_str!("../../day3_input");
    let knapsacks = build_knapsacks(input_data);

    let priorities: Vec<Priority> = knapsacks
        .iter()
        .map(|knapsack| knapsack.find_repeated())
        .map(|char| Priority::from(char))
        .collect();

    println!("{:?}", priorities.iter().map(|x| x.0).sum::<u32>());
}

fn build_knapsacks(data: &str) -> Vec<Knapsack> {
    data.lines().map(|line| Knapsack::from(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        let test_data = include_str!("../../test_inputs/day3_test");
        let knapsacks = build_knapsacks(test_data);

        assert_eq!(knapsacks.len(), 6);

        let priorities: Vec<Priority> = knapsacks
            .iter()
            .map(|knapsack| knapsack.find_repeated())
            .map(|char| Priority::from(char))
            .collect();

        assert_eq!(157, priorities.iter().map(|x| x.0).sum::<u32>());
    }
}
