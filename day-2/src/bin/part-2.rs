const BAG: &str = "12 red, 13 green, 14 blue";

fn get_count(input: &str) -> Vec<u32> {
    let mut buckets = vec![0, 0, 0]; // red, blue, green

    // input
    let input_buffer = input.chars().collect::<Vec<char>>();
    let mut pointer = 0;
    let mut value = 0;

    while pointer < input_buffer.len() {
        let c = input_buffer[pointer];

        if c.is_numeric() {
            let mut num = String::new();
            let mut cursor = 0;
            let mut next_char = input_buffer[pointer + cursor];

            while next_char.is_numeric() {
                num.push(next_char);
                cursor += 1;

                if (pointer + cursor) >= input_buffer.len() {
                    break;
                }

                next_char = input_buffer[pointer + cursor];
            }
            pointer += cursor;

            value = num.parse::<u32>().unwrap();
        }

        if c.is_alphabetic() {
            let mut color = String::new();
            let mut cursor = 0;
            let mut next_char = input_buffer[pointer + cursor];

            while next_char.is_alphabetic() {
                color.push(next_char);
                cursor += 1;

                if (pointer + cursor) >= input_buffer.len() {
                    break;
                }

                next_char = input_buffer[pointer + cursor];
            }

            match color.as_str() {
                "red" => buckets[0] += value,
                "green" => buckets[1] += value,
                "blue" => buckets[2] += value,
                _ => (),
            }

            pointer += cursor;
            continue;
        }

        pointer += 1;
    }

    buckets
}

fn compare_vec(a: &Vec<u32>, b: &Vec<u32>) -> bool {
    for i in 0..a.len() {
        if a[i] < b[i] {
            return false;
        }
    }

    true
}

fn save(input: Vec<u32>, output: &mut Vec<u32>) {
    for i in 0..input.len() {
        if input[i] > output[i] {
            output[i] = input[i];
        }
    }
}

fn possible(input: &str, condition: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        // Split string into game id and game data
        let result = line.split(":").collect::<Vec<&str>>();

        let mut min_count = vec![0, 0, 0];

        for set in result[1].split(";") {
            let set_count = get_count(set);

            save(set_count, &mut min_count);
        }

        sum += min_count[0] * min_count[1] * min_count[2];
    }

    sum
}

fn main() {
    let input_text = include_str!("./input-2.txt");
    println!("{}", possible(input_text, BAG));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(possible(input, BAG), 48);
    }

    #[test]
    fn three() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(possible(input, BAG), 1560);
    }

    #[test]
    fn full() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(possible(input, BAG), 2286);
    }
}
