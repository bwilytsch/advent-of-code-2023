// NOTE: This looks like a dynammic programming problem
fn combos(cfg: &str) -> Vec<String> {
    if cfg.is_empty() {
        return vec!["".to_string()];
    }

    let first_char = if cfg.chars().next().unwrap() == '?' {
        "#."
    } else {
        &cfg[..1]
    };

    combos(&cfg[1..])
        .into_iter()
        .flat_map(|x| first_char.chars().map(move |y| format!("{}{}", y, x)))
        .collect::<Vec<_>>()
}

fn process(input: &str) -> Result<i32, String> {
    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        // Get all possible combinations
        let mut result = line.split_whitespace();

        let conditions = result.next().unwrap();
        let pattern = result
            .next()
            .unwrap()
            .split(',')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        for test in combos(conditions) {
            let combo = test
                .split('.')
                .map(|s| s.len())
                .filter(|n| n > &0_usize)
                .collect::<Vec<_>>();

            if combo == pattern {
                sum += 1;
            }
        }
    }

    Ok(sum)
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
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(process(input).unwrap(), 21);
    }
}
