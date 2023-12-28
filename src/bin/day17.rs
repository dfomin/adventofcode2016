use std::collections::VecDeque;

use adventofcode2016::read_input;

use md5::{Digest, Md5};

struct Position {
    x: usize,
    y: usize,
    path: String,
}

impl Position {
    fn apply(&self, opens: &[char]) -> Vec<Position> {
        let mut result = vec![];
        if self.y > 0 && "bcdef".contains(opens[0]) {
            result.push(Position {
                x: self.x,
                y: self.y - 1,
                path: self.path.to_string() + "U",
            });
        }
        if self.y < 3 && "bcdef".contains(opens[1]) {
            result.push(Position {
                x: self.x,
                y: self.y + 1,
                path: self.path.to_string() + "D",
            });
        }
        if self.x > 0 && "bcdef".contains(opens[2]) {
            result.push(Position {
                x: self.x - 1,
                y: self.y,
                path: self.path.to_string() + "L",
            });
        }
        if self.x < 3 && "bcdef".contains(opens[3]) {
            result.push(Position {
                x: self.x + 1,
                y: self.y,
                path: self.path.to_string() + "R",
            });
        }
        result
    }
}

fn hash(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let hash = hasher.finalize();
    format!("{:x}", hash).to_string()
}

fn process(input: &str, shortest: bool) -> String {
    let mut paths = VecDeque::new();
    paths.push_back(Position {
        x: 0,
        y: 0,
        path: String::new(),
    });
    let mut result = String::new();
    while !paths.is_empty() {
        let current = paths.pop_front().unwrap();
        let code = input.to_string() + &current.path[..];
        let hash = hash(&code).chars().collect::<Vec<_>>();
        for position in current.apply(&hash) {
            if position.x == 3 && position.y == 3 {
                if shortest {
                    return position.path;
                } else if position.path.len() > result.len() {
                    result = position.path;
                }
            } else {
                paths.push_back(position);
            }
        }
    }
    result
}

fn part1(input: &str) -> String {
    process(input, true)
}

fn part2(input: &str) -> usize {
    process(input, false).len()
}

fn main() {
    let input = read_input(17);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day17_part1() {
        assert_eq!(&hash("hijkl")[..4], "ced9");
        assert_eq!(&hash("hijklD")[..4], "f2bc");
        assert_eq!(&hash("hijklDR")[..4], "5745");

        assert_eq!(part1("ihgpwlah"), "DDRRRD");
        assert_eq!(part1("kglvqrro"), "DDUDRLRRUDRD");
        assert_eq!(part1("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn test_day17_part2() {
        assert_eq!(part2("ihgpwlah"), 370);
        assert_eq!(part2("kglvqrro"), 492);
        assert_eq!(part2("ulqzkmiv"), 830);
    }
}
