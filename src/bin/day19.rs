use adventofcode2016::read_input;

fn part1(input: &str) -> usize {
    let total = input.parse::<usize>().unwrap();
    let mut elves = vec![1; total];
    let mut cur = 0;
    let mut next = 1;
    loop {
        elves[cur] += elves[next];
        if elves[cur] == total {
            return cur + 1;
        }
        elves[next] = 0;
        cur = (next + 1) % elves.len();
        while elves[cur] == 0 {
            cur = (cur + 1) % elves.len();
        }
        next = (cur + 1) % elves.len();
        while elves[next] == 0 {
            next = (next + 1) % elves.len();
        }
    }
}

fn part2(input: &str) -> usize {
    let total = input.parse::<usize>().unwrap();
    let mut elves: Vec<usize> = (1..=total).collect();
    let mut cur = 0;
    let mut next = total / 2;
    while elves.len() > 1 {
        elves.remove(next);
        if cur < next {
            cur = (cur + 1) % elves.len();
        } else {
            cur = cur % elves.len();
        }
        next = (cur + elves.len() / 2) % elves.len();
    }
    elves[0]
}

fn main() {
    let input = read_input(19);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day19_part1() {
        assert_eq!(part1("5"), 3);
    }

    #[test]
    fn test_day19_part2() {
        assert_eq!(part2("5"), 2);
    }
}
