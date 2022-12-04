use itertools::Itertools;

#[derive(Debug)]
struct ElfPair {
    sections: Vec<ElfSection>,
}

impl ElfPair {
    pub fn from_entry(entry: &str) -> Self {
        let sections = entry
            .split(",")
            .map(|x| ElfSection::from_entry(x))
            .collect::<Vec<ElfSection>>();

        Self { sections }
    }

    pub fn has_self_containment(&self) -> bool {
        self.sections[0].contains(&self.sections[1]) || self.sections[1].contains(&self.sections[0])
    }

    pub fn has_overlaps(&self) -> bool {
        self.sections[0].overlaps(&self.sections[1])
    }
}

#[derive(Debug)]
struct ElfSection {
    start: u32,
    end: u32,
}

impl ElfSection {
    pub fn from_entry(entry: &str) -> Self {
        let data: Vec<&str> = entry.split("-").collect();

        Self {
            start: data[0].parse::<u32>().unwrap(),
            end: data[1].parse::<u32>().unwrap(),
        }
    }

    pub fn contains(&self, other: &ElfSection) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &ElfSection) -> bool {
        println!("{:?}, {:?}: {}", self, other, self.end >= other.start);

        self.end >= other.start && self.start <= other.end
    }
}

fn main() {
    let input_data = include_str!("../../day4_input");

    let elf_pairs = input_data
        .lines()
        .map(|line| ElfPair::from_entry(line))
        .collect::<Vec<ElfPair>>();

    let first_phase = elf_pairs
        .iter()
        .map(|pair| if pair.has_self_containment() { 1 } else { 0 })
        .sum::<u32>();

    println!("{}", first_phase);

    let second_phase = elf_pairs
        .iter()
        .map(|pair| if pair.has_overlaps() { 1 } else { 0 })
        .sum::<u32>();

    println!("{}", second_phase);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_half() {
        let input_data = include_str!("../../test_inputs/day4_test");

        let elf_pairs = input_data
            .lines()
            .map(|line| ElfPair::from_entry(line))
            .collect::<Vec<ElfPair>>();

        assert_eq!(6, elf_pairs.len());

        let result = elf_pairs
            .iter()
            .map(|pair| if pair.has_self_containment() { 1 } else { 0 })
            .sum::<u32>();

        assert_eq!(result, 2);

        let result = elf_pairs
            .iter()
            .map(|pair| if pair.has_overlaps() { 1 } else { 0 })
            .sum::<u32>();

        assert_eq!(result, 4);
    }

    #[test]
    fn second_half() {}
}
