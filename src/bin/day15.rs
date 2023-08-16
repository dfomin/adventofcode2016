use adventofcode2016::read_input;

struct Disc {
    position: usize,
    total: usize,
}

fn process(discs: Vec<Disc>) -> usize {
    let mut time = (discs[0].total - discs[0].position - 1) as usize;
    loop {
        let mut all_ok = true;
        for (i, disc) in discs.iter().enumerate() {
            if (disc.position + time + i + 1) % disc.total != 0 {
                all_ok = false;
                break;
            }
        }

        if all_ok {
            return time;
        }

        time += discs[0].total;
    }
}

fn part1(input: &str) -> usize {
    process(parse_input(input))
}

fn part2(input: &str) -> usize {
    let mut discs = parse_input(input);
    discs.push(Disc {
        position: 0,
        total: 11,
    });
    process(discs)
}

fn parse_input(input: &str) -> Vec<Disc> {
    let mut discs = vec![];
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<_>>();
        let total = parts[3].parse::<usize>().unwrap();
        let position = parts[11][..parts[11].len() - 1].parse::<usize>().unwrap();
        discs.push(Disc { position, total });
    }
    discs
}

fn main() {
    let input = read_input(15);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_part1() {
        assert_eq!(
            process(vec![
                Disc {
                    position: 4,
                    total: 5
                },
                Disc {
                    position: 1,
                    total: 2
                }
            ]),
            5
        );
    }

    #[test]
    fn test_day15_part2() {}
}
