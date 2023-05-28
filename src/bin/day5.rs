use std::collections::HashMap;

use md5::{Digest, Md5};

use adventofcode2016::read_input;

fn part1(input: &String) -> String {
    let mut i = 0;
    let mut password = String::new();
    loop {
        let mut s = input.clone();
        s.push_str(&format!("{i}"));

        let mut hasher = Md5::new();
        hasher.update(s);
        let hash = hasher.finalize();
        if hash[0] == 0 && hash[1] == 0 && hash[2] >> 4 == 0 {
            password.push(if hash[2] >= 10 {
                ('a' as u8 + (hash[2] - 10) as u8) as char
            } else {
                ('0' as u8 + hash[2] as u8) as char
            });
            if password.len() == 8 {
                return password;
            }
        }

        i += 1;
    }
}

fn convert_char(value: u8) -> char {
    if value >= 10 {
        ('a' as u8 + (value - 10) as u8) as char
    } else {
        ('0' as u8 + value as u8) as char
    }
}

fn part2(input: &String) -> String {
    let mut i = 0;
    let mut map = HashMap::new();
    loop {
        let mut s = input.clone();
        s.push_str(&format!("{i}"));

        let mut hasher = Md5::new();
        hasher.update(s);
        let hash = hasher.finalize();
        if hash[0] == 0 && hash[1] == 0 && hash[2] >> 4 == 0 {
            let mut key = convert_char(hash[2]) as usize;
            if key >= '0' as usize {
                key = key - '0' as usize;
                if key < 8 {
                    if !map.contains_key(&key) {
                        map.insert(key, convert_char(hash[3] >> 4));
                        if map.len() == 8 {
                            break;
                        }
                    }
                }
            }
        }

        i += 1;
    }

    let mut password = String::new();
    for i in 0..8 {
        password.push(map[&i]);
    }

    return password;
}

fn main() {
    let input = read_input(5);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_part1() {
        assert_eq!(part1(&"abc".to_string()), "18f47a30");
    }

    #[test]
    fn test_day5_part2() {
        assert_eq!(part2(&"abc".to_string()), "05ace8e3");
    }
}
