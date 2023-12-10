fn parse_numbers(input: &str) -> Vec<u64> {
    println!("{:?}", input);
    input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn process(input: &str) -> Result<u64, &str> {
    let result = input
        .lines()
        .into_iter()
        .map(|x| parse_numbers(x))
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|v| {
            let times = &v[0];
            let record_distances = &v[1];

            times
                .iter()
                .enumerate()
                .map(|(idx, max_time)| {
                    let record_distance = record_distances[idx];
                    let mut count = 0;

                    for i in 1..*max_time {
                        let distance = (max_time - i) * i;

                        if distance > record_distance {
                            count += 1;
                        }
                    }

                    count
                })
                .fold(1, |acc, s| acc * s)
        })
        .sum();

    Ok(result)
}

fn main() {
    let input = include_str!("./input.txt");
    if let Ok(result) = process(input) {
        println!("{:?}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(process(input).unwrap(), 288);
    }
}
