use itertools::Itertools;
use std::collections::VecDeque;

use adventofcode2016::read_input;

fn bfs(field: &Vec<Vec<char>>, x: usize, y: usize, matrix: &mut Vec<Vec<usize>>) {
    let mut positions = VecDeque::new();
    positions.push_back((x, y, 0));
    let mut visited = vec![vec![false; field[0].len()]; field.len()];
    let orig = field[y][x].to_digit(10).unwrap() as usize;
    visited[y][x] = true;
    while !positions.is_empty() {
        let (x, y, steps) = positions.pop_front().unwrap();
        if let Some(number) = field[y][x].to_digit(10) {
            let number = number as usize;
            if number != orig && matrix[number][orig] == 0 {
                matrix[orig][number] = steps;
                matrix[number][orig] = steps;
            }
        }
        for shift in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_x = (x as i32 + shift.0) as usize;
            let new_y = (y as i32 + shift.1) as usize;
            if field[new_y][new_x] != '#' && !visited[new_y][new_x] {
                visited[new_y][new_x] = true;
                positions.push_back((new_x, new_y, steps + 1));
            }
        }
    }
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    let field: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|x| x.trim().chars().collect())
        .collect();
    let mut max = 0usize;
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if field[i][j] != '.' && field[i][j] != '#' {
                max = max.max(field[i][j].to_digit(10).unwrap() as usize);
            }
        }
    }
    let mut matrix = vec![vec![0; max + 1]; max + 1];
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if field[i][j] != '.' && field[i][j] != '#' {
                bfs(&field, j, i, &mut matrix);
            }
        }
    }
    matrix
}

fn process(input: &str, back: bool) -> usize {
    let matrix = parse(input);
    let mut result = usize::MAX;
    for perm in (0..matrix.len() - 1).permutations(matrix.len() - 1) {
        let mut cur = matrix[0][perm[0] + 1];
        for i in 1..perm.len() {
            cur += matrix[perm[i] + 1][perm[i - 1] + 1];
        }
        if back {
            cur += matrix[perm[perm.len() - 1] + 1][0];
        }
        result = result.min(cur);
    }
    result
}

fn part1(input: &str) -> usize {
    process(input, false)
}

fn part2(input: &str) -> usize {
    process(input, true)
}

fn main() {
    let input = read_input(24);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
        ###########
        #0.1.....2#
        #.#######.#
        #4.......3#
        ###########
    ";

    #[test]
    fn test_day24_part1() {
        assert_eq!(part1(INPUT), 14);
    }

    #[test]
    fn test_day24_part2() {}
}
