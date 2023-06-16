use std::collections::HashMap;

use md5::{Digest, Md5};

use adventofcode2016::read_input;

struct MD5Helper {
    cache: HashMap<String, String>,
    count_hash: usize,
}

impl MD5Helper {
    fn new(count_hash: usize) -> MD5Helper {
        return MD5Helper {
            cache: HashMap::new(),
            count_hash,
        };
    }

    fn md5_hash(&mut self, input: &str, i: usize) -> String {
        let s = format!("{}{}", input, i);
        if let Some(value) = self.cache.get(&s) {
            return value.clone();
        }

        let mut hash = s.clone();
        for _ in 0..self.count_hash {
            let mut hasher = Md5::new();
            hasher.update(hash.clone());
            hash = format!("{:x}", hasher.finalize());
        }
        self.cache.insert(s, hash.clone());
        return hash;
    }
}

fn find_three(hash: &str) -> Option<char> {
    let mut last_char = ' ';
    let mut counter = 0;
    for ch in hash.chars() {
        if ch == last_char {
            counter += 1;
            if counter == 3 {
                return Some(ch);
            }
        } else {
            last_char = ch;
            counter = 1;
        }
    }

    return None;
}

fn check_five(hash: &str, char_to_check: char) -> bool {
    let mut counter = 0;
    for ch in hash.chars() {
        if ch == char_to_check {
            counter += 1;
            if counter == 5 {
                return true;
            }
        } else {
            counter = 0;
        }
    }

    return false;
}

fn process(input: &str, count_hash: usize, key_count: usize) -> usize {
    let mut i = 0;
    let mut key_counter = 0;
    let mut helper = MD5Helper::new(count_hash);
    loop {
        let hash = helper.md5_hash(input, i);
        if let Some(ch) = find_three(&hash) {
            for j in i + 1..i + 1001 {
                let new_hash = helper.md5_hash(input, j);
                if check_five(&new_hash, ch) {
                    key_counter += 1;
                    if key_counter == key_count {
                        return i;
                    } else {
                        break;
                    }
                }
            }
        }

        i += 1;
    }
}

fn part1(input: &str) -> usize {
    return process(input, 1, 64);
}

fn part2(input: &str) -> usize {
    return process(input, 2017, 64);
}

fn main() {
    let input = read_input(14);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14_part1() {
        assert_eq!(find_three("aabbaabcccaaa"), Some('c'));
        assert!(check_five("aaaaa", 'a'));
        assert!(!check_five("aaaaa", 'b'));
        assert!(check_five("aaaabbbbccccc", 'c'));
        assert!(check_five("aaaaaaaaaaa", 'a'));
        assert!(!check_five("aaaabaaaabaaa", 'a'));
        assert_eq!(process("abc", 1, 1), 39);
        assert_eq!(process("abc", 1, 2), 92);
        assert_eq!(part1("abc"), 22728);
    }

    #[test]
    fn test_day14_part2() {
        assert_eq!(process("abc", 2017, 1), 10);
        assert_eq!(part2("abc"), 22551);
    }
}
