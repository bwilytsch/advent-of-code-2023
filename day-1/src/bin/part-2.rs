fn resolve(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut result: Vec<u32> = vec![];
        let buffer = line.chars().collect::<Vec<char>>();
        let mut index = 0;

        while index < buffer.len() {
            let c = buffer[index];

            if c.is_digit(10) {
                result.push(c.to_digit(10).unwrap());
                index += 1;
                continue;
            }

            let mut cursor = 0;
            let mut word = String::new();

            while cursor < 6 && index + cursor < buffer.len() {
                let letter = buffer[index + cursor];

                if letter.is_digit(10) {
                    break;
                }

                word.push(letter);

                match word.as_str() {
                    "nine" => {
                        result.push(9);
                        break;
                    }
                    "eight" => {
                        result.push(8);
                        break;
                    }
                    "seven" => {
                        result.push(7);
                        break;
                    }
                    "six" => {
                        result.push(6);
                        break;
                    }
                    "five" => {
                        result.push(5);
                        break;
                    }
                    "four" => {
                        result.push(4);
                        break;
                    }
                    "three" => {
                        result.push(3);
                        break;
                    }
                    "two" => {
                        result.push(2);
                        break;
                    }
                    "one" => {
                        result.push(1);
                        break;
                    }
                    "zero" => {
                        result.push(0);
                        break;
                    }
                    _ => {
                        cursor += 1;
                    }
                }
            }

            index += 1
        }

        let first = result.first().unwrap_or(&0);
        let last = result.last().unwrap_or(&0);

        let combined = format!("{}{}", first, last);
        sum += combined.parse::<u32>().unwrap();
    }

    return sum;
}

fn main() {
    let input_two = include_str!("./input-2.txt");
    println!("Output Two: {}", resolve(input_two));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_one() {
        let test_input = "eightwothree";
        assert_eq!(resolve(test_input), 83);
    }

    #[test]
    fn part_two_full() {
        let test_input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(resolve(test_input), 281);
    }
}
