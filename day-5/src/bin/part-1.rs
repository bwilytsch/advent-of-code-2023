use indicatif::ProgressIterator;

#[derive(Clone, Debug)]
struct Map {
    destination: u64,
    source: u64,
    range: u64,
}

fn parse_numbers(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

// Find the lowest location number from the seed.
fn process(input: &str) -> Result<u64, ()> {
    let lines: Vec<&str> = input
        .lines()
        .filter(|l| l.chars().any(char::is_numeric) || l.is_empty())
        .collect();

    let seed_numbers = parse_numbers(lines.first().unwrap());

    let mut maps: Vec<Vec<Map>> = vec![];

    for i in 1..lines.len() {
        let line = lines[i];

        if line.len() == 0 {
            maps.push(vec![]);
        } else {
            let numbers = parse_numbers(line);

            let destination = numbers[0];
            let source = numbers[1];
            let range = numbers[2];

            if let Some(current_map) = maps.last_mut() {
                current_map.push(Map {
                    destination,
                    source,
                    range,
                });
            };
        }
    }

    seed_numbers
        .into_iter()
        .progress()
        .map(|sn| {
            maps.clone().into_iter().fold(sn, |mut acc, map| {
                for m in &map {
                    if acc >= m.source && acc < m.source + m.range {
                        acc = acc + m.destination - m.source;
                        break;
                    }
                }

                acc
            })
        })
        .min()
        .ok_or(())
}

fn main() {
    let input = include_str!("./input-1.txt");
    if let Ok(output) = process(input) {
        println!("{}", output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = include_str!("./test.txt");
        assert_eq!(process(input).unwrap(), 35);
    }
}
