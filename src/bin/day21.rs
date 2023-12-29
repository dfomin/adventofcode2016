use core::panic;

use itertools::Itertools;
use regex::Regex;

use adventofcode2016::read_input;

enum Operation {
    SwapPosition(usize, usize),
    SwapLetters(char, char),
    Rotate(i32),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

fn parse(input: &str) -> Vec<Operation> {
    let mut operations = vec![];
    for line in input.trim().lines() {
        let line = line.trim();
        if let Some(caps) = Regex::new(r"^swap position (.*) with position (.*)$")
            .unwrap()
            .captures(line)
        {
            let values: [&str; 2] = caps.extract().1;
            operations.push(Operation::SwapPosition(
                values[0].parse().unwrap(),
                values[1].parse().unwrap(),
            ));
        } else if let Some(caps) = Regex::new(r"^swap letter (.) with letter (.)$")
            .unwrap()
            .captures(line)
        {
            let values: [&str; 2] = caps.extract().1;
            operations.push(Operation::SwapLetters(
                values[0].chars().next().unwrap(),
                values[1].chars().next().unwrap(),
            ));
        } else if let Some(caps) = Regex::new(r"^rotate left (.*) steps?$")
            .unwrap()
            .captures(line)
        {
            let values: [&str; 1] = caps.extract().1;
            operations.push(Operation::Rotate(-values[0].parse::<i32>().unwrap()));
        } else if let Some(caps) = Regex::new(r"^rotate right (.*) steps?$")
            .unwrap()
            .captures(line)
        {
            let values: [&str; 1] = caps.extract().1;
            operations.push(Operation::Rotate(values[0].parse::<i32>().unwrap()));
        } else if let Some(caps) = Regex::new(r"^rotate based on position of letter (.*)$")
            .unwrap()
            .captures(line)
        {
            let values: [&str; 1] = caps.extract().1;
            operations.push(Operation::RotateLetter(values[0].chars().next().unwrap()));
        } else if let Some(caps) = Regex::new(r"^reverse positions (.*) through (.*)$")
            .unwrap()
            .captures(line)
        {
            let values: [&str; 2] = caps.extract().1;
            operations.push(Operation::Reverse(
                values[0].parse().unwrap(),
                values[1].parse().unwrap(),
            ));
        } else if let Some(caps) = Regex::new(r"^move position (.*) to position (.*)$")
            .unwrap()
            .captures(line)
        {
            let values: [&str; 2] = caps.extract().1;
            operations.push(Operation::Move(
                values[0].parse().unwrap(),
                values[1].parse().unwrap(),
            ));
        } else {
            panic!("Unexpected");
        }
    }
    operations
}

fn process(input: &str, line: &str) -> String {
    let mut chars = line.chars().collect::<Vec<_>>();
    let operations = parse(input);
    for operation in operations {
        match operation {
            Operation::SwapPosition(x, y) => {
                chars.swap(x, y);
            }
            Operation::SwapLetters(a, b) => {
                let mut x = 0;
                let mut y = 0;
                for (i, &ch) in chars.iter().enumerate() {
                    if ch == a {
                        x = i;
                    } else if ch == b {
                        y = i;
                    }
                }
                chars.swap(x, y);
            }
            Operation::Rotate(x) => {
                let copy = chars.clone();
                for i in 0..chars.len() {
                    chars[(i as i32 + x + copy.len() as i32) as usize % copy.len()] = copy[i];
                }
            }
            Operation::RotateLetter(a) => {
                let index = chars.iter().position(|&r| r == a).unwrap();
                let mut x = index as i32 + 1;
                if index >= 4 {
                    x += 1;
                }
                let copy = chars.clone();
                for i in 0..chars.len() {
                    chars[(i as i32 + x + copy.len() as i32) as usize % copy.len()] = copy[i];
                }
            }
            Operation::Reverse(x, y) => {
                let mut i = x;
                let mut j = y;
                while i < j {
                    chars.swap(i, j);
                    i += 1;
                    j -= 1;
                }
            }
            Operation::Move(x, y) => {
                let ch = chars.remove(x);
                chars.insert(y, ch);
            }
        }
    }
    chars.iter().collect()
}

fn part1(input: &str, line: &str) -> String {
    process(input, line)
}

fn part2(input: &str, line: &str) -> String {
    for p in line.chars().permutations(line.len()) {
        if line == process(input, &p.iter().collect::<String>()[..]) {
            return p.iter().collect::<String>();
        }
    }
    String::new()
}

fn main() {
    let input = read_input(21);

    println!("{}", part1(&input, "abcdefgh"));
    println!("{}", part2(&input, "fbgdceah"));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        swap position 4 with position 0
        swap letter d with letter b
        reverse positions 0 through 4
        rotate left 1 step
        move position 1 to position 4
        move position 3 to position 0
        rotate based on position of letter b
        rotate based on position of letter d
    ";

    #[test]
    fn test_day21_part1() {
        assert_eq!(part1(INPUT, "abcde"), "decab");
    }

    #[test]
    fn test_day21_part2() {}
}
