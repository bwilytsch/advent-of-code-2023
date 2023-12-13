use std::collections::HashMap;

// TODO: Lookup LCM and CRT in more depth
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn from_char(c: char) -> Result<Instruction, String> {
        match c {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(format!("Invalid inruction: {}", c)),
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn walk(
    instructions: &[Instruction],
    nodes_map: &HashMap<String, Vec<String>>,
) -> Result<usize, String> {
    let mut lcm_value = 1;
    let runners = nodes_map
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();

    for r in runners.iter() {
        let mut steps = 0_usize;
        let mut current_node = *r;

        while !current_node.ends_with('Z') {
            let node = nodes_map.get(current_node).ok_or("Invalid node")?;
            let choice = instructions[steps % instructions.len()];

            match choice {
                Instruction::Left => current_node = &node[0],
                Instruction::Right => current_node = &node[1],
            }

            steps += 1;
        }

        lcm_value = lcm(lcm_value, steps);
    }

    Ok(lcm_value)
}

fn process(input: &str) -> Result<usize, String> {
    let lines = input.lines().collect::<Vec<_>>();

    let mut nodes_map: HashMap<String, Vec<String>> = HashMap::new();
    let instructions = lines[0]
        .chars()
        .filter_map(|c| Instruction::from_char(c).ok())
        .collect::<Vec<_>>();

    for line in input.lines().skip(2) {
        let (value, nodes) = line.split_once(" = ").ok_or("Invalid line")?;
        let parsed_nodes = nodes
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        nodes_map.insert(value.to_string(), parsed_nodes);
    }

    walk(&instructions, &nodes_map)
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
    use rstest::rstest;

    #[test]
    fn base_case() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(process(input).unwrap(), 6);
    }

    #[rstest]
    #[case(["AAC", "BBA"], false)]
    #[case(["AAZ", "BBZ"], true)]
    fn check_base_case(#[case] path_nodes: [&str; 2], #[case] expected: bool) {
        assert_eq!(path_nodes.iter().all(|n| n.ends_with('Z')), expected);
    }
}
