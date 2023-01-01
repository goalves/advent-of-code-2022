fn main() {
    let input = include_str!("../../inputs/day13");
    // let input = include_str!("../../inputs/test/day13");

    let packet_pairs: Vec<Vec<Packet>> = input
        .split("\n\n")
        .into_iter()
        .map(|lines| lines.trim())
        .map(|lines| lines.split('\n').map(Packet::from).collect::<Vec<Packet>>())
        .collect();

    let mut correct_pairs_index = Vec::new();

    for (index, pair) in packet_pairs.iter().enumerate() {
        if pair[0] < pair[1] {
            correct_pairs_index.push(index + 1);
        }
    }

    let mut flattened = packet_pairs.into_iter().flatten().collect::<Vec<Packet>>();
    let first_divider = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let second_divider = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    flattened.push(first_divider.clone());
    flattened.push(second_divider.clone());

    flattened.sort();

    let first_divider_index = flattened.iter().position(|x| x == &first_divider).unwrap() + 1;
    let second_divider_index = flattened.iter().position(|x| x == &second_divider).unwrap() + 1;

    println!("{}", first_divider_index * second_divider_index)
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::List(_), Packet::Number(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Packet::Number(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(&other),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Packet {
    fn from(line: &str) -> Self {
        if &line[0..1] == "[" {
            let mut list_stack: i32 = 0;
            let check_char = |c| {
                if c == '[' {
                    list_stack += 1
                } else if c == ']' {
                    list_stack -= 1
                }
                c == ',' && list_stack == 0
            };

            Packet::List(
                line[1..line.len() - 1]
                    .split(check_char)
                    .filter_map(|s| {
                        if !s.is_empty() {
                            Some(Packet::from(s))
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        } else {
            Packet::Number(line.parse().unwrap())
        }
    }
}
