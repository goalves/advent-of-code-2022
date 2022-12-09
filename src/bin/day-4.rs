#[derive(Debug)]
struct ElfPair {
    sections: Vec<ElfSection>,
}

impl ElfPair {
    pub fn has_self_containment(&self) -> bool {
        self.sections[0].contains(&self.sections[1]) || self.sections[1].contains(&self.sections[0])
    }

    pub fn has_overlaps(&self) -> bool {
        self.sections[0].overlaps(&self.sections[1])
    }
}

impl From<&str> for ElfPair {
    fn from(entry: &str) -> Self {
        let sections = entry
            .split(',')
            .map(ElfSection::from)
            .collect::<Vec<ElfSection>>();

        Self { sections }
    }
}

#[derive(Debug)]
struct ElfSection {
    start: u32,
    end: u32,
}

impl ElfSection {
    pub fn contains(&self, other: &ElfSection) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &ElfSection) -> bool {
        self.end >= other.start && self.start <= other.end
    }
}

impl From<&str> for ElfSection {
    fn from(entry: &str) -> Self {
        let data: Vec<&str> = entry.split('-').collect();

        Self {
            start: data[0].parse::<u32>().unwrap(),
            end: data[1].parse::<u32>().unwrap(),
        }
    }
}

fn main() {
    let elf_pairs = include_str!("../../day4_input").lines().map(ElfPair::from);

    let first_phase = elf_pairs
        .clone()
        .map(|pair| u32::from(pair.has_self_containment()))
        .sum::<u32>();

    println!("Number of elf pairs with full containments: {first_phase}");

    let second_phase = elf_pairs
        .map(|pair| u32::from(pair.has_overlaps()))
        .sum::<u32>();

    println!("Number of elf pairs with overlaps: {second_phase}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_half() {
        let elf_pairs = include_str!("../../test_inputs/day4_test")
            .lines()
            .map(|line| ElfPair::from(line))
            .collect::<Vec<ElfPair>>();

        assert_eq!(6, elf_pairs.len());

        let result = elf_pairs
            .iter()
            .map(|pair| if pair.has_self_containment() { 1 } else { 0 })
            .sum::<u32>();

        assert_eq!(result, 2);
    }

    #[test]
    fn second_half() {
        let elf_pairs = include_str!("../../test_inputs/day4_test")
            .lines()
            .map(|line| ElfPair::from(line))
            .collect::<Vec<ElfPair>>();

        assert_eq!(6, elf_pairs.len());

        let result = elf_pairs
            .iter()
            .map(|pair| if pair.has_overlaps() { 1 } else { 0 })
            .sum::<u32>();

        assert_eq!(result, 4);
    }
}
