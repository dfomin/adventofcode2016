use adventofcode2016::read_input;

fn process(input: &String, is_max: bool) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    let mut stats = vec![vec![0; 26]; lines[0].chars().count()];
    for line in lines {
        for (i, ch) in line.chars().enumerate() {
            let index = ch as usize - 'a' as usize;
            stats[i][index] += 1;
        }
    }

    let mut result = String::new();
    for v in stats {
        let value_to_compare;
        if is_max {
            value_to_compare = v.iter().max().unwrap();
        } else {
            value_to_compare = v.iter().filter(|x| **x > 0).min().unwrap();
        }
        for (i, value) in v.iter().enumerate() {
            if value == value_to_compare {
                result.push(('a' as u8 + i as u8) as char);
                break;
            }
        }
    }

    return result;
}

fn part1(input: &String) -> String {
    return process(input, true);
}

fn part2(input: &String) -> String {
    return process(input, false);
}

fn main() {
    let input = read_input(6);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1() {
        let input = "eedadn\n\
                     drvtee\n\
                     eandsr\n\
                     raavrd\n\
                     atevrs\n\
                     tsrnev\n\
                     sdttsa\n\
                     rasrtv\n\
                     nssdts\n\
                     ntnada\n\
                     svetve\n\
                     tesnvt\n\
                     vntsnd\n\
                     vrdear\n\
                     dvrsen\n\
                     enarar";
        assert_eq!(part1(&input.to_string()), "easter");
    }

    #[test]
    fn test_day6_part2() {
        let input = "eedadn\n\
                     drvtee\n\
                     eandsr\n\
                     raavrd\n\
                     atevrs\n\
                     tsrnev\n\
                     sdttsa\n\
                     rasrtv\n\
                     nssdts\n\
                     ntnada\n\
                     svetve\n\
                     tesnvt\n\
                     vntsnd\n\
                     vrdear\n\
                     dvrsen\n\
                     enarar";
        assert_eq!(part2(&input.to_string()), "advent");
    }
}
