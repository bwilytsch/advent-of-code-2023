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

fn possible(input: &str, condition: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        // Split string into game id and game data
        let result = line.split(":").collect::<Vec<&str>>();

        // println!("{:?}", line);

        let game_id = result[0]
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        let bag_count = get_count(condition);
        let mut should_add = true;

        println!("{:?}", result[1]);
        for set in result[1].split(";") {
            let set_count = get_count(set);

            println!("{:?} {:?}", set_count, bag_count);

            if !compare_vec(&bag_count, &set_count) {
                should_add = false;
                break;
            }
        }

        println!("should_add: {}", should_add);

        if should_add {
            sum += game_id;
        }
    }

    sum
}

fn main() {
    let input_txt = include_str!("./input-1.txt");
    println!("{}", possible(input_txt, "12 red, 13 green, 14 blue"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one() {
        let games = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let bag = "12 red, 13 green, 14 blue";
        assert_eq!(possible(games, bag), 1);
    }

    #[test]
    fn full() {
        let games = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let bag = "12 red, 13 green, 14 blue";
        assert_eq!(possible(games, bag), 8);
    }
}
