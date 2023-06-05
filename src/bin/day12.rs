use adventofcode2016::read_input;

enum Instruction {
    Assign(i32, usize),
    Copy(usize, usize),
    Increment(usize),
    Decrement(usize),
    JumpNonZero(usize, i32),
    Jump(i32),
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

fn process(input: &str, mut registers: Vec<i32>) -> i32 {
    let mut instructions = vec![];
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        instructions.push(match parts[0] {
            "cpy" => {
                if let Ok(value) = parts[1].parse::<i32>() {
                    Instruction::Assign(value, parse_register(parts[2]))
                } else {
                    Instruction::Copy(parse_register(parts[1]), parse_register(parts[2]))
                }
            }
            "inc" => Instruction::Increment(parse_register(parts[1])),
            "dec" => Instruction::Decrement(parse_register(parts[1])),
            "jnz" => {
                if let Ok(_) = parts[1].parse::<i32>() {
                    Instruction::Jump(parts[2].parse::<i32>().unwrap())
                } else {
                    Instruction::JumpNonZero(
                        parse_register(parts[1]),
                        parts[2].parse::<i32>().unwrap(),
                    )
                }
            }
            _ => panic!("Unexpected command"),
        });
    }

    let mut index = 0;
    while index < instructions.len() {
        let command = &instructions[index];
        match command {
            Instruction::Assign(v, reg) => {
                registers[*reg] = *v;
                index += 1;
            }
            Instruction::Copy(src, trg) => {
                registers[*trg] = registers[*src];
                index += 1;
            }
            Instruction::Increment(reg) => {
                registers[*reg] += 1;
                index += 1;
            }
            Instruction::Decrement(reg) => {
                registers[*reg] -= 1;
                index += 1;
            }
            Instruction::Jump(v) => index = (index as i32 + *v) as usize,
            Instruction::JumpNonZero(reg, v) => {
                if registers[*reg] != 0 {
                    index = (index as i32 + *v) as usize
                } else {
                    index += 1;
                }
            }
        }
    }

    return registers[0];
}

fn part1(input: &str) -> i32 {
    process(input, vec![0, 0, 0, 0])
}

fn part2(input: &str) -> i32 {
    process(input, vec![0, 0, 1, 0])
}

fn main() {
    let input = read_input(12);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_part1() {
        let input = "cpy 41 a\n\
                     inc a\n\
                     inc a\n\
                     dec a\n\
                     jnz a 2\n\
                     dec a\n";
        assert_eq!(part1(input), 42);
    }

    #[test]
    fn test_day12_part2() {}
}
