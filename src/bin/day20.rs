use adventofcode2016::read_input;

struct BlockList {
    ranges: Vec<(usize, usize)>,
}

impl BlockList {
    fn add_range(&mut self, mut range: (usize, usize)) {
        let mut i = 0;
        while i < self.ranges.len() {
            if self.ranges[i].0 > range.1 || self.ranges[i].1 < range.0 {
                i += 1;
            } else {
                range = (range.0.min(self.ranges[i].0), range.1.max(self.ranges[i].1));
                self.ranges.remove(i);
            }
        }
        self.ranges.push(range);
    }

    fn min(&self) -> usize {
        let mut number = 0;
        loop {
            if let Some(max) = self.check(number) {
                number = max + 1;
            } else {
                return number;
            }
        }
    }

    fn check(&self, number: usize) -> Option<usize> {
        for range in &self.ranges {
            if range.0 <= number && number <= range.1 {
                return Some(range.1);
            }
        }
        None
    }
}

fn part1(input: &str) -> usize {
    let mut ranges = vec![];
    for line in input.trim().lines() {
        let parts = line.trim().split('-').collect::<Vec<_>>();
        ranges.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
    }
    let block_list = BlockList { ranges };
    block_list.min()
}

fn part2(input: &str) -> usize {
    let mut block_list = BlockList { ranges: vec![] };
    for line in input.trim().lines() {
        let parts = line.trim().split('-').collect::<Vec<_>>();
        block_list.add_range((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
    }
    let mut blocked = 0;
    for range in block_list.ranges {
        blocked += range.1 - range.0 + 1;
    }
    u32::MAX as usize - blocked + 1
}

fn main() {
    let input = read_input(20);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        5-8
        0-2
        4-7
    ";

    #[test]
    fn test_day20_part1() {
        assert_eq!(part1(INPUT), 3);
    }

    #[test]
    fn test_day20_part2() {}
}
