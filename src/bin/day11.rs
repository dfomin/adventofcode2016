use std::collections::{HashMap, HashSet};

use adventofcode2016::read_input;

#[derive(Clone, Debug)]
struct State {
    floors: Vec<Vec<bool>>,
    current_floor: usize,
}

impl State {
    fn is_correct(&self, floor: usize) -> bool {
        let mut has_gen = false;
        let mut has_chip = false;
        for (i, &v) in self.floors[floor].iter().enumerate() {
            if v {
                if i % 2 == 0 {
                    has_gen = true;
                } else {
                    if !self.floors[floor][(i - 1) as usize] {
                        has_chip = true;
                    }
                }
            }
        }

        return !has_chip || !has_gen;
    }

    fn movable_items(&self) -> Vec<Vec<usize>> {
        let mut indices = vec![];
        for (i, &v) in self.floors[self.current_floor].iter().enumerate() {
            if v {
                indices.push(i);
            }
        }

        let mut result = vec![];
        for &i in &indices {
            result.push(vec![i]);
        }

        for i in 0..indices.len() {
            for j in i + 1..indices.len() {
                result.push(vec![indices[i], indices[j]]);
            }
        }

        return result;
    }

    fn possible_states(&self) -> Vec<State> {
        let mut result = vec![];
        for item in self.movable_items() {
            for next_floor in self.possible_next_floors() {
                let mut state = self.clone();
                state.move_items(item.clone(), next_floor);
                if state.is_correct(self.current_floor) && state.is_correct(state.current_floor) {
                    result.push(state);
                }
            }
        }

        return result;
    }

    fn possible_next_floors(&self) -> Vec<usize> {
        if self.current_floor == 0 {
            return vec![self.current_floor + 1];
        }

        if self.current_floor == 3 {
            return vec![(self.current_floor - 1) as usize];
        }

        return vec![(self.current_floor - 1) as usize, self.current_floor + 1];
    }

    fn move_items(&mut self, items: Vec<usize>, floor: usize) {
        for index in items {
            self.floors[self.current_floor][index] = false;
            self.floors[floor][index] = true;
        }

        self.current_floor = floor;
    }

    fn is_finished(&self) -> bool {
        return self.current_floor == 3
            && self.floors[self.current_floor]
                .iter()
                .map(|x| if *x { 1 } else { 0 })
                .sum::<usize>()
                == self.floors[self.current_floor].len();
    }

    fn hash(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.current_floor.to_string());
        for floor in &self.floors {
            result.push_str(
                &floor
                    .iter()
                    .map(|x| if *x { '1' } else { '0' })
                    .collect::<String>(),
            );
        }

        return result;
    }
}

fn parse_input(input: &str) -> State {
    let mut power_types: HashMap<String, usize> = HashMap::new();
    let mut chips: HashMap<String, usize> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let parts = line.split(" generator").collect::<Vec<_>>();
        for j in 0..parts.len() - 1 {
            let words = parts[j].split_whitespace().collect::<Vec<_>>();
            let power_type = words.last().unwrap();
            power_types.insert(power_type.to_string(), i);
        }

        let parts = line.split("-compatible").collect::<Vec<_>>();
        for j in 0..parts.len() - 1 {
            let words = parts[j].split_whitespace().collect::<Vec<_>>();
            let chip = words.last().unwrap();
            chips.insert(chip.to_string(), i);
        }
    }

    let mut floors = vec![vec![false; power_types.len() * 2]; 4];
    for (i, t) in power_types.keys().enumerate() {
        floors[power_types[t]][i * 2] = true;
        floors[chips[t]][i * 2 + 1] = true;
    }

    return State {
        floors,
        current_floor: 0,
    };
}

fn process(state: State) -> i32 {
    let mut states = vec![state.clone()];
    let mut set: HashSet<String> = HashSet::new();
    set.insert(state.hash());
    let mut step = 0;

    loop {
        let mut next_states = vec![];
        for state in states {
            if state.is_finished() {
                return step;
            } else {
                for next_state in state.possible_states() {
                    let hash = next_state.hash();
                    if !set.contains(&hash) {
                        set.insert(hash);
                        next_states.push(next_state);
                    }
                }
            }
        }

        states = next_states;

        step += 1;
    }
}

fn part1(state: State) -> i32 {
    return process(state);
}

fn part2(state: State) -> i32 {
    let mut floors = vec![vec![false; state.floors[0].len() + 4]; 4];
    for (i, floor) in state.floors.iter().enumerate() {
        for (j, &v) in floor.iter().enumerate() {
            if v {
                floors[i][j] = true;
            }
        }
    }

    for i in floors[0].len() - 4..floors[0].len() {
        floors[0][i] = true;
    }

    let state = State {
        floors,
        current_floor: 0,
    };

    return process(state);
}

fn main() {
    let input = read_input(11);

    let state = parse_input(&input);

    println!("{}", part1(state.clone()));
    println!("{}", part2(state));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_part1() {
        let state = State {
            floors: vec![
                vec![false, true, false, true],
                vec![true, false, false, false],
                vec![false, false, true, false],
                vec![false, false, false, false],
            ],
            current_floor: 0,
        };

        for i in 0..4 {
            assert!(state.is_correct(i));
        }

        assert_eq!(state.movable_items(), vec![vec![1], vec![3], vec![1, 3]]);

        assert_eq!(state.possible_next_floors(), vec![1]);

        assert_eq!(state.possible_states().len(), 1);

        assert_eq!(part1(state), 11);
    }

    #[test]
    fn test_day11_part2() {}
}
