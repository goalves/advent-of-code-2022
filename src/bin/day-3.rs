use itertools::Itertools;

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

    pub fn has_item(&self, item: char) -> bool {
        self.first_half.contains(item) || self.second_half.contains(item)
    }

    pub fn backpack(&self) -> (&str, &str) {
        (self.first_half, self.second_half)
    }
}

#[derive(Debug)]
struct KnapsackGroup<'a> {
    knapsacks: Vec<&'a Knapsack<'a>>,
}

impl<'a> From<Vec<&'a Knapsack<'a>>> for KnapsackGroup<'a> {
    fn from(value: Vec<&'a Knapsack<'a>>) -> Self {
        Self { knapsacks: value }
    }
}

impl<'a> KnapsackGroup<'a> {
    pub fn find_badge(&self) -> char {
        let iter = self.knapsacks[0]
            .backpack()
            .0
            .chars()
            .chain(self.knapsacks[0].backpack().1.chars());

        for char in iter {
            if self.knapsacks[1].has_item(char) && self.knapsacks[2].has_item(char) {
                return char;
            }
        }

        panic!("should always have a badge between group")
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

fn build_knapsacks(data: &str) -> Vec<Knapsack> {
    data.lines().map(|line| Knapsack::from(line)).collect()
}

fn find_repeated_from_knapsacks(knapsacks: &Vec<Knapsack>) -> Vec<Priority> {
    knapsacks
        .iter()
        .map(|knapsack| knapsack.find_repeated())
        .map(|char| Priority::from(char))
        .collect()
}

fn calculate_badges_from_knapsacks(knapsacks: &Vec<Knapsack>) -> Vec<Priority> {
    knapsacks
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let knapsack_group = KnapsackGroup::from(chunk.collect::<Vec<&Knapsack>>());
            let badge = knapsack_group.find_badge();

            Priority::from(badge)
        })
        .collect::<Vec<Priority>>()
}

fn main() {
    let input_data = include_str!("../../day3_input");
    let knapsacks = build_knapsacks(input_data);

    let mut priorities: Vec<Priority> = find_repeated_from_knapsacks(&knapsacks);
    println!(
        "first phase: {:?}",
        priorities.iter().map(|x| x.0).sum::<u32>()
    );

    priorities = calculate_badges_from_knapsacks(&knapsacks);
    println!(
        "second phase: {:?}",
        priorities.iter().map(|x| x.0).sum::<u32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        let test_data = include_str!("../../test_inputs/day3_test");
        let knapsacks = build_knapsacks(test_data);

        assert_eq!(knapsacks.len(), 6);

        let priorities: Vec<Priority> = find_repeated_from_knapsacks(&knapsacks);
        assert_eq!(157, priorities.iter().map(|x| x.0).sum::<u32>());
    }

    #[test]
    fn second_half() {
        let test_data = include_str!("../../test_inputs/day3_test");
        let knapsacks = build_knapsacks(test_data);
        assert_eq!(knapsacks.len(), 6);

        let priorities = calculate_badges_from_knapsacks(&knapsacks);
        assert_eq!(70, priorities.iter().map(|x| x.0).sum::<u32>());
    }
}
