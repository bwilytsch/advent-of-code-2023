fn parse_numbers(input: &str) -> Result<Vec<i32>, &str> {
    let mut output: Vec<i32> = vec![];

    let buffer = input.chars().collect::<Vec<char>>();
    let mut pointer = 0;

    while pointer < buffer.len() {
        let c = buffer[pointer];

        if c.is_numeric() {
            let mut next_char = c;
            let mut result = String::new();
            let mut cursor = 0;

            while next_char.is_numeric() {
                result.push(next_char);
                cursor += 1;

                if pointer + cursor >= buffer.len() {
                    break;
                }

                next_char = buffer[pointer + cursor];
            }

            output.push(result.parse::<i32>().unwrap());

            pointer += cursor;
            continue;
        }

        pointer += 1;
    }

    Ok(output)
}

fn process(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let result: Vec<&str> = line.split(":").collect();
        let numbers: Vec<&str> = result[1].split("|").collect();

        if let Ok(winning_numbers) = parse_numbers(*numbers.first().unwrap()) {
            if let Ok(my_numbers) = parse_numbers(*numbers.last().unwrap()) {
                let filtered_numbers = my_numbers
                    .iter()
                    .filter_map(|n| {
                        if winning_numbers.contains(n) {
                            return Some(*n);
                        }

                        None
                    })
                    .collect::<Vec<i32>>();

                let mut points = 0;

                for i in 0..filtered_numbers.len() {
                    if i == 0 {
                        points = 1;
                    } else {
                        points *= 2;
                    }
                }

                sum += points;
            }
        }
    }

    sum
}

fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process(input), 13);
    }
}
