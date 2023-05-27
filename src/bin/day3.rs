use adventofcode2016::read_input;

fn check_triangle(values: &Vec<i32>) -> bool {
    values[0] + values[1] > values[2]
        && values[0] + values[2] > values[1]
        && values[1] + values[2] > values[0]
}

fn part1(input: &String) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if check_triangle(&values) {
            result += 1;
        }
    }

    return result;
}

fn part2(input: &String) -> i32 {
    let mut result = 0;
    let mut columns = vec![vec![], vec![], vec![]];
    for (line_id, line) in input.lines().enumerate() {
        let values = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        for i in 0..columns.len() {
            columns[i].push(values[i]);
        }
        if line_id % 3 == 2 {
            for i in 0..3 {
                if check_triangle(&vec![
                    columns[i][line_id - 2],
                    columns[i][line_id - 1],
                    columns[i][line_id],
                ]) {
                    result += 1;
                }
            }
        }
    }

    return result;
}

fn main() {
    let input = read_input(3);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1() {
        assert_eq!(part1(&"5 10 25\n5 5 5".to_string()), 1);
    }

    #[test]
    fn test_day3_part2() {
        assert_eq!(part2(&"5 10 25\n5 5 5\n5 1 20".to_string()), 1);
    }
}
