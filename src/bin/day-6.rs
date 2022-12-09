use std::collections::VecDeque;

fn find_sequence(data: &str, expected_length: usize) -> usize {
    let chars: Vec<char> = data.chars().collect();
    let mut packets = VecDeque::new();

    for (index, char) in chars.iter().enumerate() {
        if packets.len() == expected_length {
            return index;
        }

        while packets.contains(char) {
            packets.pop_front();
        }

        packets.push_back(chars[index]);
    }

    panic!("shouldn't get here if the sequence is possible")
}

fn main() {
    let lenght_4 = find_sequence(include_str!("../../day6_input"), 4);
    println!("{lenght_4}");

    let length_14 = find_sequence(include_str!("../../day6_input"), 14);
    println!("{length_14}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(7, find_sequence("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(5, find_sequence("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, find_sequence("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, find_sequence("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, find_sequence("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }

    #[test]
    fn test_2() {
        assert_eq!(19, find_sequence("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(23, find_sequence("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(23, find_sequence("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(29, find_sequence("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
        assert_eq!(26, find_sequence("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
    }
}
