use adventofcode2016::read_input;

fn process(original_state: &str, max_size: usize) -> String {
    let mut state = original_state.chars().map(|x| x == '1').collect::<Vec<_>>();
    while state.len() < max_size {
        let new_half = state.iter().rev().map(|x| !x).collect::<Vec<_>>();
        state.push(false);
        state.extend(new_half);
    }

    state.truncate(max_size);

    while state.len() % 2 == 0 {
        let mut i = 0;
        while i < state.len() {
            state[i / 2] = state[i] == state[i + 1];
            i += 2;
        }
        state.truncate(state.len() / 2);
    }

    state
        .into_iter()
        .map(|x| if x { '1' } else { '0' })
        .collect()
}

fn part1(input: &str) -> String {
    process(input, 272)
}

fn part2(input: &str) -> String {
    process(input, 35651584)
}

fn main() {
    let input = read_input(16);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_part1() {
        assert_eq!(process("10000", 20), "01100");
    }
}
