use std::ops::Range;

use indicatif::ProgressIterator;

#[derive(Clone, Debug)]
struct Map {
    destination: i64,
    source: i64,
    range: i64,
}

fn parse_numbers(input: &str) -> Vec<i64> {
    input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

fn translate(maps: &Vec<Vec<Map>>, from: i64) -> i64 {
    maps.into_iter().fold(from, |mut acc, map| {
        for m in map {
            // This is currently hardcoded
            if acc >= m.destination && acc < m.destination + m.range {
                acc = acc - m.destination + m.source;
                break;
            }
        }

        acc
    })
}

// Find the lowest location number from the seed.
fn process(input: &str) -> Result<i64, ()> {
    let lines: Vec<&str> = input
        .lines()
        .filter(|l| l.chars().any(char::is_numeric) || l.is_empty())
        .collect();

    let seed_number_pairs = parse_numbers(lines.first().unwrap());

    println!("Generating seed numbers...");

    let seed_numbers = seed_number_pairs
        .chunks(2)
        .map(|v| Range {
            start: v[0],
            end: v[0] + v[1],
        })
        .collect::<Vec<_>>();

    let mut maps: Vec<Vec<Map>> = vec![];

    println!("Creating maps...");

    for i in (1..lines.len()).progress() {
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
                    source,
                    destination,
                    range,
                });
            };
        }
    }

    // This could be highger
    let mut location = 1_i64;

    // Using reversed maps to speed up the search.
    // Trying to find the lowest location matchin one of the seed ranges
    maps.reverse();

    println!("Finding location...");

    'outer: loop {
        let result = translate(&maps, location);
        for sr in &seed_numbers {
            if sr.contains(&result) {
                break 'outer;
            }
        }

        location += 1;
    }

    Ok(location)
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
        assert_eq!(process(input).unwrap(), 46);
    }
}
