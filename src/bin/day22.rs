use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use regex::Regex;

use adventofcode2016::read_input;

#[derive(Debug, Clone)]
struct Node {
    x: usize,
    y: usize,
    used: usize,
    avail: usize,
}

#[derive(Debug, Clone)]
struct Position {
    goal: (usize, usize),
    empty: (usize, usize),
    steps: usize,
}

impl Position {
    fn encode(&self) -> (usize, usize, usize, usize) {
        (self.goal.0, self.goal.1, self.empty.0, self.empty.1)
    }
}

fn parse(input: &str) -> Vec<Node> {
    let mut nodes = vec![];
    for line in input.trim().lines().skip(2) {
        if let Some(caps) =
            Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%$")
                .unwrap()
                .captures(line.trim())
        {
            let parts: [&str; 6] = caps.extract().1;
            nodes.push(Node {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
                used: parts[3].parse().unwrap(),
                avail: parts[4].parse().unwrap(),
            });
        } else {
            panic!("Unexpected");
        }
    }
    nodes
}

fn part1(input: &str) -> usize {
    let nodes = parse(input);
    let mut result = 0;
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            if nodes[i].used > 0 && nodes[i].used <= nodes[j].avail {
                result += 1;
            }
            if nodes[j].used > 0 && nodes[j].used <= nodes[i].avail {
                result += 1;
            }
        }
    }
    result
}

fn part2(input: &str) -> usize {
    let nodes = parse(input);
    let mut max_x = 0;
    let mut max_y = 0;
    for node in &nodes {
        max_x = max_x.max(node.x);
        max_y = max_y.max(node.y);
    }
    let mut grid = vec![
        vec![
            Node {
                x: 0,
                y: 0,
                used: 0,
                avail: 0
            };
            max_x + 1
        ];
        max_y + 1
    ];
    for node in &nodes {
        grid[node.y][node.x] = node.clone();
    }
    let goal = (max_x, 0);
    let mut empty = (0, 0);
    let mut usable = vec![vec![true; max_x + 1]; max_y + 1];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut ok = true;
            if i > 0 {
                if grid[i - 1][j].used + grid[i - 1][j].avail < grid[i][j].used {
                    ok = false;
                }
            }
            if i < max_y {
                if grid[i + 1][j].used + grid[i + 1][j].avail < grid[i][j].used {
                    ok = false;
                }
            }
            if j > 0 {
                if grid[i][j - 1].used + grid[i][j - 1].avail < grid[i][j].used {
                    ok = false;
                }
            }
            if j < max_x {
                if grid[i][j + 1].used + grid[i][j + 1].avail < grid[i][j].used {
                    ok = false;
                }
            }
            usable[i][j] = ok;
            if grid[i][j].used == 0 {
                empty = (j, i);
            }
        }
    }
    let position = Position {
        goal,
        empty,
        steps: 0,
    };
    let mut positions = VecDeque::new();
    positions.push_back(position.clone());
    let mut visited = HashMap::new();
    let mut record = usize::MAX;
    while !positions.is_empty() {
        let position = positions.pop_front().unwrap();
        let encoded = position.encode();
        if visited.contains_key(&encoded) && visited[&encoded] <= position.steps {
            continue;
        }
        visited.insert(encoded, position.steps);
        for shift in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            if (position.empty.0 == 0 && shift.0 == -1)
                || (position.empty.1 == 0 && shift.1 == -1)
                || (position.empty.0 == max_x && shift.0 == 1)
                || (position.empty.1 == max_y && shift.1 == 1)
            {
                continue;
            }
            let new_empty = (
                (position.empty.0 as i32 + shift.0) as usize,
                (position.empty.1 as i32 + shift.1) as usize,
            );
            if !usable[new_empty.1][new_empty.0] {
                continue;
            }
            if new_empty == position.goal {
                if position.empty == (0, 0) {
                    record = record.min(position.steps + 1);
                    continue;
                }
                positions.push_back(Position {
                    goal: position.empty,
                    empty: position.goal,
                    steps: position.steps + 1,
                });
            } else {
                positions.push_back(Position {
                    goal: position.goal,
                    empty: new_empty,
                    steps: position.steps + 1,
                })
            }
        }
    }
    record
}

fn main() {
    let input = read_input(22);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        root@ebhq-gridcenter# df -h
        Filesystem            Size  Used  Avail  Use%
        /dev/grid/node-x0-y0   10T    8T     2T   80%
        /dev/grid/node-x0-y1   11T    6T     5T   54%
        /dev/grid/node-x0-y2   32T   28T     4T   87%
        /dev/grid/node-x1-y0    9T    7T     2T   77%
        /dev/grid/node-x1-y1    8T    0T     8T    0%
        /dev/grid/node-x1-y2   11T    7T     4T   63%
        /dev/grid/node-x2-y0   10T    6T     4T   60%
        /dev/grid/node-x2-y1    9T    8T     1T   88%
        /dev/grid/node-x2-y2    9T    6T     3T   66%
    ";

    #[test]
    fn test_day22_part1() {}

    #[test]
    fn test_day22_part2() {
        assert_eq!(part2(INPUT), 7);
    }
}
