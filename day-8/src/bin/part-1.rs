use std::collections::HashMap;

use rstest::rstest;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn from_char(c: char) -> Instruction {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => !panic!("Invalid instruction: {}", c),
        }
    }
}

fn process(input: &str) -> Result<usize, String> {
    let lines = input.split("\n").collect::<Vec<_>>();

    let mut nodes_map: HashMap<String, Vec<String>> = HashMap::new();
    let instructions = lines[0]
        .chars()
        .map(Instruction::from_char)
        .collect::<Vec<_>>();

    for line in input.lines().skip(2) {
        let (value, nodes) = line.split_once(" = ").unwrap();
        let parsed_nodes = nodes
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        nodes_map.insert(value.to_string(), parsed_nodes);
    }

    let mut current_node = "AAA";
    let mut steps = 0;

    while current_node != "ZZZ" {
        let node = nodes_map.get(current_node).unwrap();
        let choice = instructions[steps % instructions.len()];

        match choice {
            Instruction::Left => {
                current_node = &node[0];
            }
            Instruction::Right => {
                current_node = &node[1];
            }
        }

        steps += 1;
    }

    Ok(steps)
}

fn main() {
    let input = include_str!("./input.txt");
    match process(input) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process(input).unwrap(), 2);
    }

    #[rstest]
    #[case("LLR", vec![Instruction::Left, Instruction::Left, Instruction::Right])]
    #[case("LRR", vec![Instruction::Left, Instruction::Right, Instruction::Right])]
    fn parse_instructions(#[case] input: &str, #[case] expected: Vec<Instruction>) {
        assert_eq!(
            input
                .chars()
                .map(Instruction::from_char)
                .collect::<Vec<_>>(),
            expected
        );
    }
}
