use adventofcode2016::read_input;

#[derive(Debug, Clone)]
enum Argument {
    Register(String),
    Value(i32),
}

impl Argument {
    fn value(&self, registers: &Vec<i32>) -> i32 {
        match self {
            Argument::Register(r) => registers[parse_register(&r)],
            Argument::Value(v) => *v,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Copy(Argument, Argument),
    Increment(Argument),
    Decrement(Argument),
    JumpNonZero(Argument, Argument),
    Toggle(Argument),
    Out(Argument),
}

impl Instruction {
    fn toggle(&mut self) {
        *self = match self {
            Instruction::Copy(v1, v2) => Instruction::JumpNonZero(v1.clone(), v2.clone()),
            Instruction::Increment(reg) => Instruction::Decrement(reg.clone()),
            Instruction::Decrement(reg) => Instruction::Increment(reg.clone()),
            Instruction::JumpNonZero(v1, v2) => Instruction::Copy(v1.clone(), v2.clone()),
            Instruction::Toggle(reg) => Instruction::Increment(reg.clone()),
            Instruction::Out(reg) => Instruction::Increment(reg.clone()),
        }
    }
}

fn parse_register(register: &str) -> usize {
    match register {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Unexpected register"),
    }
}

fn parse_argument(s: &str) -> Argument {
    if let Ok(value) = s.parse::<i32>() {
        Argument::Value(value)
    } else {
        Argument::Register(s.to_string())
    }
}

fn process(input: &str, mut registers: Vec<i32>, max_length: usize) -> bool {
    let mut instructions = vec![];
    for line in input.trim().lines() {
        let parts = line.trim().split_whitespace().collect::<Vec<_>>();
        instructions.push(match parts[0] {
            "cpy" => Instruction::Copy(parse_argument(parts[1]), parse_argument(parts[2])),
            "inc" => Instruction::Increment(parse_argument(parts[1])),
            "dec" => Instruction::Decrement(parse_argument(parts[1])),
            "jnz" => Instruction::JumpNonZero(parse_argument(parts[1]), parse_argument(parts[2])),
            "tgl" => Instruction::Toggle(parse_argument(parts[1])),
            "out" => Instruction::Out(parse_argument(parts[1])),
            _ => panic!("Unexpected command"),
        });
    }

    let mut index = 0;
    let mut counter = 0;
    let mut last_value = 1;
    while index < instructions.len() {
        let command = instructions[index].clone();
        match command {
            Instruction::Copy(src, trg) => {
                let v = src.value(&registers);
                if let Argument::Register(reg_name) = trg {
                    registers[parse_register(&reg_name)] = v;
                } else {
                    panic!("Unexpected");
                }
                index += 1;
            }
            Instruction::Increment(reg) => {
                if let Argument::Register(reg_name) = reg {
                    registers[parse_register(&reg_name)] += 1;
                    index += 1;
                } else {
                    panic!("Unexpected");
                }
            }
            Instruction::Decrement(reg) => {
                if let Argument::Register(reg_name) = reg {
                    registers[parse_register(&reg_name)] -= 1;
                    index += 1;
                } else {
                    panic!("Unexpected");
                }
            }
            Instruction::JumpNonZero(reg, v) => {
                if reg.value(&registers) != 0 {
                    index = (index as i32 + v.value(&registers)) as usize
                } else {
                    index += 1;
                }
            }
            Instruction::Toggle(reg) => {
                let i = (index as i32 + reg.value(&registers)) as usize;
                if i < instructions.len() {
                    instructions[i].toggle();
                }
                index += 1;
            }
            Instruction::Out(reg) => {
                let value = reg.value(&registers);
                match value {
                    0..=1 => {
                        if last_value + value == 1 {
                            last_value = value;
                            counter += 1;
                            if counter == max_length {
                                return true;
                            }
                        } else {
                            return false;
                        }
                    }
                    _ => return false,
                }
                index += 1;
            }
        }
    }

    return false;
}

fn part1(input: &str) -> i32 {
    let mut i = 0;
    loop {
        if process(input, vec![i, 0, 0, 0], 10) {
            return i;
        }
        i += 1;
    }
}

fn part2(input: &str) -> i32 {
    0
}

fn main() {
    let input = read_input(25);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day23_part1() {}

    #[test]
    fn test_day23_part2() {}
}
