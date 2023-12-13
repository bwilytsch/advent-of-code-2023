fn process(input: &str) -> Result<i32, String> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut sum = 0;

    for line in lines {
        let init = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut steps = vec![];
        let mut numbers = init.clone();

        while !numbers.iter().all(|n| n == &0) {
            let mut new_numbers = vec![];

            for i in 0..(numbers.len() - 1) {
                let diff = numbers[i + 1] - numbers[i];
                new_numbers.push(diff);
            }

            steps.push(new_numbers.clone());
            numbers = new_numbers;
        }

        let total_diff = steps.iter().map(|s| s.last().unwrap()).sum::<i32>();

        sum += init.last().unwrap() + total_diff;
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
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(process(input).unwrap(), 114);
    }
}
