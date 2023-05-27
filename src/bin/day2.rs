use adventofcode2016::read_input;

fn part1(input: &String) -> String {
    let mut i = 1;
    let mut j = 1;
    let mut result = String::new();
    let table = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    for line in input.lines() {
        for ch in line.chars() {
            match ch {
                'U' => i = if i > 0 { i - 1 } else { i },
                'R' => j = if j < 2 { j + 1 } else { j },
                'D' => i = if i < 2 { i + 1 } else { i },
                'L' => j = if j > 0 { j - 1 } else { j },
                _ => panic!("Unexpected char"),
            }
        }

        result.push(table[i][j]);
    }

    return result;
}

fn part2(input: &String) -> String {
    let mut i = 2i32;
    let mut j = 0i32;
    let mut result = String::new();
    let table = vec![
        vec!['X', 'X', '1', 'X', 'X'],
        vec!['X', '2', '3', '4', 'X'],
        vec!['5', '6', '7', '8', '9'],
        vec!['X', 'A', 'B', 'C', 'X'],
        vec!['X', 'X', 'D', 'X', 'X'],
    ];

    for line in input.lines() {
        for ch in line.chars() {
            match ch {
                'U' => {
                    i = if (i - 3).abs() + (j - 2).abs() <= 2 {
                        i - 1
                    } else {
                        i
                    }
                }
                'R' => {
                    j = if (i - 2).abs() + (j - 1).abs() <= 2 {
                        j + 1
                    } else {
                        j
                    }
                }
                'D' => {
                    i = if (i - 1).abs() + (j - 2).abs() <= 2 {
                        i + 1
                    } else {
                        i
                    }
                }
                'L' => {
                    j = if (i - 2).abs() + (j - 3).abs() <= 2 {
                        j - 1
                    } else {
                        j
                    }
                }
                _ => panic!("Unexpected char"),
            }
        }

        result.push(table[i as usize][j as usize]);
    }

    return result;
}

fn main() {
    let input = read_input(2);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD".to_string();
        assert_eq!(part1(&input), "1985");
    }

    #[test]
    fn test_day2_part2() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD".to_string();
        assert_eq!(part2(&input), "5DB3");
    }
}
