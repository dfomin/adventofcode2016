use adventofcode2016::read_input;

fn new_line(line: &Vec<char>) -> Vec<char> {
    let mut result = vec![];
    result.push(if line[1] == '^' { '^' } else { '.' });
    for i in 1..line.len() - 1 {
        result.push(if line[i + 1] != line[i - 1] { '^' } else { '.' });
    }
    result.push(if line[line.len() - 2] == '^' {
        '^'
    } else {
        '.'
    });
    result
}

fn process(line: &str, lines_count: usize) -> Vec<Vec<char>> {
    let mut result = vec![line.chars().collect()];
    while result.len() < lines_count {
        result.push(new_line(&result.last().unwrap()));
    }
    result
}

fn count(input: &str, lines_count: usize) -> usize {
    let field = process(input, lines_count);
    field
        .iter()
        .map(|x| x.iter().filter(|y| **y == '.').count())
        .sum()
}

fn part1(input: &str) -> usize {
    count(input, 40)
}

fn part2(input: &str) -> usize {
    count(input, 400000)
}

fn main() {
    let input = read_input(18);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const OUTPUT1: &str = "
        ..^^.
        .^^^^
        ^^..^
    ";

    const OUTPUT2: &str = "
        .^^.^.^^^^
        ^^^...^..^
        ^.^^.^.^^.
        ..^^...^^^
        .^^^^.^^.^
        ^^..^.^^..
        ^^^^..^^^.
        ^..^^^^.^^
        .^^^..^.^^
        ^^.^^^..^^
    ";

    #[test]
    fn test_day18_part1() {
        assert_eq!(
            process("..^^.", 3),
            OUTPUT1
                .trim()
                .lines()
                .map(|x| x.trim().chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        );

        assert_eq!(
            process(".^^.^.^^^^", 10),
            OUTPUT2
                .trim()
                .lines()
                .map(|x| x.trim().chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        );

        assert_eq!(count(".^^.^.^^^^", 10), 38);
    }

    #[test]
    fn test_day18_part2() {}
}
