use std::collections::HashMap;

use adventofcode2016::read_input;

fn part1(input: &str, chips: (i32, i32)) -> i32 {
    let mut bots = HashMap::new();
    let mut map = HashMap::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts[0] == "value" {
            let bot_id = parts.last().unwrap().parse::<i32>().unwrap();
            bots.entry(bot_id)
                .or_insert(vec![])
                .push(parts[1].parse::<i32>().unwrap());
        } else {
            let bot_id = parts[1].parse::<i32>().unwrap();
            let low = if parts[5] == "bot" {
                parts[6].parse::<i32>().unwrap()
            } else {
                -1
            };
            let high = if parts[10] == "bot" {
                parts[11].parse::<i32>().unwrap()
            } else {
                -1
            };
            map.insert(bot_id, (low, high));
        }
    }

    loop {
        let mut bot_id = -1;
        for (&key, value) in bots.iter() {
            if value.len() == 2 {
                bot_id = key;
                break;
            }
        }

        let value = &bots[&bot_id];
        let min = *value.iter().min().unwrap();
        let max = *value.iter().max().unwrap();
        if min == chips.0 && max == chips.1 {
            return bot_id;
        }

        let (low, high) = map[&bot_id];
        bots.entry(bot_id).and_modify(|x| x.clear());

        if low >= 0 {
            bots.entry(low).or_insert(vec![]).push(min);
        }
        if high >= 0 {
            bots.entry(high).or_insert(vec![]).push(max);
        }
    }
}

fn part2(input: &str) -> i32 {
    let mut bots = HashMap::new();
    let mut map = HashMap::new();
    let mut output: HashMap<i32, i32> = HashMap::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts[0] == "value" {
            let bot_id = parts.last().unwrap().parse::<i32>().unwrap();
            bots.entry(bot_id)
                .or_insert(vec![])
                .push(parts[1].parse::<i32>().unwrap());
        } else {
            let bot_id = parts[1].parse::<i32>().unwrap();
            let low = if parts[5] == "bot" {
                parts[6].parse::<i32>().unwrap()
            } else {
                -parts[6].parse::<i32>().unwrap() - 1
            };
            let high = if parts[10] == "bot" {
                parts[11].parse::<i32>().unwrap()
            } else {
                -parts[11].parse::<i32>().unwrap() - 1
            };
            map.insert(bot_id, (low, high));
        }
    }

    loop {
        let mut bot_id = -1;
        for (&key, value) in bots.iter() {
            if value.len() == 2 {
                bot_id = key;
                break;
            }
        }

        if bot_id == -1 {
            break;
        }

        let value = &bots[&bot_id];
        let min = *value.iter().min().unwrap();
        let max = *value.iter().max().unwrap();

        let (low, high) = map[&bot_id];
        bots.entry(bot_id).and_modify(|x| x.clear());

        if low >= 0 {
            bots.entry(low).or_insert(vec![]).push(min);
        } else {
            output.insert(-low - 1, min);
        }

        if high >= 0 {
            bots.entry(high).or_insert(vec![]).push(max);
        } else {
            output.insert(-high - 1, max);
        }
    }

    return output[&0] * output[&1] * output[&2];
}

fn main() {
    let input = read_input(10);

    println!("{}", part1(&input, (17, 61)));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_part1() {
        let input = "value 5 goes to bot 2\n\
                     bot 2 gives low to bot 1 and high to bot 0\n\
                     value 3 goes to bot 1\n\
                     bot 1 gives low to output 1 and high to bot 0\n\
                     bot 0 gives low to output 2 and high to output 0\n\
                     value 2 goes to bot 2";

        assert_eq!(part1(&input, (2, 5)), 2);
    }

    #[test]
    fn test_day10_part2() {}
}
