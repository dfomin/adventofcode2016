use core::panic;
use std::collections::HashSet;

use adventofcode2016::read_input;

struct Position {
    x: i32,
    y: i32,
    orientation: usize,
}

fn walk(input: &String, mut visited: Option<HashSet<String>>) -> i32 {
    let mut pos = Position {
        x: 0,
        y: 0,
        orientation: 0,
    };

    if let Some(ref mut visited) = visited {
        visited.insert(format!("{}_{}", 0, 0));
    }

    for command in input.split(", ") {
        let turn = command.chars().nth(0).unwrap();
        match turn {
            'L' => pos.orientation = (pos.orientation + 3) % 4,
            'R' => pos.orientation = (pos.orientation + 1) % 4,
            _ => panic!("Unexpexted turn"),
        }

        let steps = command[1..].parse::<i32>().unwrap();

        if let Some(ref mut visited) = visited {
            for _ in 0..steps {
                match pos.orientation {
                    0 => pos.y += 1,
                    1 => pos.x += 1,
                    2 => pos.y -= 1,
                    3 => pos.x -= 1,
                    _ => panic!("Unexpected orientation"),
                }

                let key = format!("{}_{}", pos.x, pos.y);
                if visited.contains(&key) {
                    return pos.x.abs() + pos.y.abs();
                } else {
                    visited.insert(key);
                }
            }
        } else {
            match pos.orientation {
                0 => pos.y += steps,
                1 => pos.x += steps,
                2 => pos.y -= steps,
                3 => pos.x -= steps,
                _ => panic!("Unexpected orientation"),
            }
        }
    }

    return pos.x.abs() + pos.y.abs();
}

fn part1(input: &String) -> i32 {
    return walk(input, None);
}

fn part2(input: &String) -> i32 {
    return walk(input, Some(HashSet::new()));
}

fn main() {
    let input = read_input(1);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        assert_eq!(part1(&"R2, L3".to_string()), 5);
        assert_eq!(part1(&"R2, R2, R2".to_string()), 2);
        assert_eq!(part1(&"R5, L5, R5, R3".to_string()), 12);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(part2(&"R8, R4, R4, R8".to_string()), 4);
    }
}
