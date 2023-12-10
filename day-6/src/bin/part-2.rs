fn process(input: &str) -> Result<u64, &str> {
    let result = input
        .lines()
        .into_iter()
        .map(|x| {
            x.split_ascii_whitespace()
                .filter(|s| s.chars().all(char::is_numeric))
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|v| {
            let time = &v[0].join("").parse::<u64>().unwrap();
            let record_distance = &v[1].join("").parse::<u64>().unwrap();

            let mut count = 0;

            for i in 1..*time {
                let distance = (time - i) * i;

                if distance > *record_distance {
                    count += 1;
                }
            }

            count
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
        assert_eq!(process(input).unwrap(), 71503);
    }
}
