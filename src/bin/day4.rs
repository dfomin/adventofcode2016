use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use adventofcode2016::read_input;

struct CharStats {
    ch: char,
    count: i32,
}

impl Ord for CharStats {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.count > other.count {
            return Ordering::Greater;
        } else if self.count < other.count {
            return Ordering::Less;
        }

        return self.ch.cmp(&other.ch).reverse();
    }
}

impl PartialOrd for CharStats {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for CharStats {
    fn eq(&self, other: &Self) -> bool {
        return self.cmp(other) == Ordering::Equal;
    }
}

impl Eq for CharStats {}

fn check_room(line: &str) -> (i32, String) {
    let parts = line.split("[").collect::<Vec<_>>();
    let checksum = parts[1][..parts[1].len() - 1]
        .to_string()
        .chars()
        .collect::<Vec<_>>();
    let parts = parts[0].split("-").collect::<Vec<_>>();
    let sector_id = parts[parts.len() - 1].parse::<i32>().unwrap();
    let code = parts[..parts.len() - 1].join("-");

    let mut counter: HashMap<char, i32> = HashMap::new();
    for ch in code.chars() {
        if ch != '-' {
            *counter.entry(ch).or_insert(0) += 1;
        }
    }

    let mut heap = BinaryHeap::new();
    for (ch, count) in counter {
        heap.push(CharStats { ch, count });
    }

    for i in 0..5 {
        if let Some(char_stats) = heap.pop() {
            if char_stats.ch != checksum[i] {
                return (0, String::new());
            }
        } else {
            return (0, String::new());
        }
    }

    return (sector_id, decode_room(code, sector_id));
}

fn process_char(ch: char, sector_id: i32) -> char {
    if ch == '-' {
        return ' ';
    }

    ((((ch as u8 - 'a' as u8) as i32 + sector_id) % 26) as u8 + 'a' as u8) as char
}

fn decode_room(room_name: String, sector_id: i32) -> String {
    room_name
        .chars()
        .map(|x| process_char(x, sector_id))
        .collect()
}

fn part1(input: &String) -> i32 {
    return input.lines().map(|x| check_room(x).0).sum();
}

fn part2(input: &String) -> i32 {
    for line in input.lines() {
        let (sector_id, decoded) = check_room(line);
        if decoded == "northpole object storage" {
            return sector_id;
        }
    }

    return 0;
}

fn main() {
    let input = read_input(4);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_part1() {
        assert_eq!(check_room("aaaaa-bbb-z-y-x-123[abxyz]").0, 123);
        assert_eq!(check_room("a-b-c-d-e-f-g-h-987[abcde]").0, 987);
        assert_eq!(check_room("not-a-real-room-404[oarel]").0, 404);
        assert_eq!(check_room("totally-real-room-200[decoy]").0, 0);
    }

    #[test]
    fn test_day4_part2() {
        assert_eq!(
            decode_room("qzmt-zixmtkozy-ivhz".to_string(), 343),
            "very encrypted name"
        );
    }
}
