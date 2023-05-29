use std::collections::HashSet;

use adventofcode2016::read_input;

fn has_pattern(line: &str) -> bool {
    let chars = line.chars().collect::<Vec<_>>();
    for i in 2..chars.len() - 1 {
        if chars[i] == chars[i - 1] && chars[i + 1] == chars[i - 2] && chars[i] != chars[i - 2] {
            return true;
        }
    }

    return false;
}

fn get_aba(line: &str, hash_set: &mut HashSet<String>) {
    let chars = line.chars().collect::<Vec<_>>();
    for i in 1..chars.len() - 1 {
        if chars[i] != chars[i - 1] && chars[i - 1] == chars[i + 1] {
            hash_set.insert(vec![chars[i], chars[i - 1], chars[i]].iter().collect());
        }
    }
}

fn check(line: &str) -> bool {
    let parts = line.split(&['[', ']']).collect::<Vec<_>>();
    let mut i = 1;
    while i < parts.len() {
        if has_pattern(parts[i]) {
            return false;
        }

        i += 2;
    }

    i = 0;
    while i < parts.len() {
        if has_pattern(parts[i]) {
            return true;
        }

        i += 2;
    }

    return false;
}

fn has_ssl(line: &str) -> bool {
    let parts = line.split(&['[', ']']).collect::<Vec<_>>();
    let mut i = 1;
    let mut hash_set = HashSet::new();
    while i < parts.len() {
        get_aba(parts[i], &mut hash_set);

        i += 2;
    }

    i = 0;
    while i < parts.len() {
        for aba in &hash_set {
            if parts[i].find(aba).is_some() {
                return true;
            }
        }

        i += 2;
    }

    return false;
}

fn part1(input: &String) -> usize {
    return input.lines().filter(|x| check(x)).count();
}

fn part2(input: &String) -> usize {
    return input.lines().filter(|x| has_ssl(x)).count();
}

fn main() {
    let input = read_input(7);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_part1() {
        assert_eq!(has_pattern("abba"), true);
        assert_eq!(has_pattern("mnop"), false);
        assert_eq!(has_pattern("aaaa"), false);
        assert_eq!(has_pattern("ioxxoj"), true);
        assert_eq!(has_pattern("abaa"), false);

        assert_eq!(check("abba[mnop]qrst"), true);
        assert_eq!(check("abcd[bddb]xyyx"), false);
        assert_eq!(check("aaaa[qwer]tyui"), false);
        assert_eq!(check("ioxxoj[asdfgh]zxcvbn"), true);

        assert_eq!(check("abvd[asdfgh]zxcvbn[bcda]oppo"), true);
        assert_eq!(check("abvd[asdfgh]zxcvbn[bccb]oppo"), false);
    }

    #[test]
    fn test_day7_part2() {
        assert_eq!(has_ssl("aba[bab]xyz"), true);
        assert_eq!(has_ssl("xyx[xyx]xyx"), false);
        assert_eq!(has_ssl("aaa[kek]eke"), true);
        assert_eq!(has_ssl("zazbz[bzb]cdb"), true);
    }
}
