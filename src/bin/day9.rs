use adventofcode2016::read_input;

fn process_line(line: &str, recursive: bool) -> usize {
    let chars = line.chars().collect::<Vec<_>>();
    let mut result = 0;
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '(' {
            let mut j = i + 1;
            while chars[j] != ')' {
                j += 1;
            }

            let parts = chars[i + 1..j]
                .iter()
                .collect::<String>()
                .split('x')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            i = j + 1;

            if !recursive {
                result += parts[0] * parts[1];
            } else {
                result += process_line(&line[i..i + parts[0]], true) * parts[1];
            }

            i += parts[0];
        } else {
            result += 1;
            i += 1;
        }
    }

    return result;
}

fn part1(input: &str) -> usize {
    return process_line(input, false);
}

fn part2(input: &str) -> usize {
    return process_line(input, true);
}

fn main() {
    let input = read_input(9);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_part1() {
        assert_eq!(part1(&"ADVENT"), "ADVENT".len());
        assert_eq!(part1(&"A(1x5)BC"), "ABBBBBC".len());
        assert_eq!(part1(&"(3x3)XYZ"), "XYZXYZXYZ".len());
        assert_eq!(part1(&"A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG".len());
        assert_eq!(part1(&"(6x1)(1x3)A"), "(1x3)A".len());
        assert_eq!(part1(&"X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY".len());
    }

    #[test]
    fn test_day9_part2() {
        assert_eq!(part2(&"(3x3)XYZ"), "XYZXYZXYZ".len());
        assert_eq!(part2(&"X(8x2)(3x3)ABCY"), "XABCABCABCABCABCABCY".len());
        assert_eq!(part2(&"(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
        assert_eq!(
            part2(&"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
