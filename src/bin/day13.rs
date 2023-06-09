use std::collections::HashMap;

use adventofcode2016::read_input;

struct State {
    input: usize,
    x: usize,
    y: usize,
    steps: usize,
}

impl State {
    fn new(input: usize) -> State {
        return State {
            input,
            x: 1,
            y: 1,
            steps: 0,
        };
    }

    fn check_state(&self, x: usize, y: usize, cache: &HashMap<String, usize>) -> Option<State> {
        if !self.is_wall(x, y)
            && *cache.get(&format!("{}_{}", x, y)).unwrap_or(&usize::MAX) > self.steps + 1
        {
            return Some(State {
                input: self.input,
                x,
                y,
                steps: self.steps + 1,
            });
        }

        return None;
    }

    fn next_states(&self, cache: &HashMap<String, usize>) -> Vec<State> {
        let mut result = vec![];

        if let Some(state) = self.check_state(self.x + 1, self.y, cache) {
            result.push(state);
        }

        if let Some(state) = self.check_state(self.x, self.y + 1, cache) {
            result.push(state);
        }

        if self.x > 0 {
            if let Some(state) = self.check_state((self.x - 1) as usize, self.y, cache) {
                result.push(state);
            }
        }

        if self.y > 0 {
            if let Some(state) = self.check_state(self.x, (self.y - 1) as usize, cache) {
                result.push(state);
            }
        }

        return result;
    }

    fn is_wall(&self, x: usize, y: usize) -> bool {
        let s = x * x + 3 * x + 2 * x * y + y + y * y + self.input;
        return s.count_ones() % 2 == 1;
    }
}

fn process(input: usize, x: usize, y: usize, max_steps: usize) -> (usize, usize) {
    let mut states = vec![State::new(input)];
    let mut cache: HashMap<String, usize> = HashMap::new();
    cache.insert("1_1".to_string(), 0);

    let mut record = max_steps;

    while !states.is_empty() {
        let mut new_states = vec![];
        for state in states.into_iter() {
            for new_state in state.next_states(&mut cache) {
                if new_state.x == x && new_state.y == y {
                    record = record.min(new_state.steps);
                } else if new_state.steps <= record {
                    cache.insert(format!("{}_{}", new_state.x, new_state.y), new_state.steps);
                    new_states.push(new_state);
                }
            }
        }

        states = new_states;
    }

    return (record, cache.len());
}

fn part1(input: usize, x: usize, y: usize) -> usize {
    return process(input, x, y, usize::MAX).0;
}

fn part2(input: usize) -> usize {
    return process(input, 50, 50, 50).1;
}

fn main() {
    let input = read_input(13).parse().unwrap();

    println!("{}", part1(input, 31, 39));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_part1() {
        let state = State::new(10);
        assert!(!state.is_wall(1, 1));
        assert!(state.is_wall(2, 1));
        assert!(state.is_wall(1, 3));

        assert_eq!(part1(10, 7, 4), 11);
    }

    #[test]
    fn test_day13_part2() {}
}
