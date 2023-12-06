use std::collections::HashSet;

fn process(input: &str) -> Result<i32, &str> {
    let mut sum = 0;

    for line in input.lines() {
        let result: Vec<&str> = line.split(':').collect();
        let cards: Vec<&str> = result[1].split('|').collect();

        let winnum: HashSet<i32> = cards
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mynum: HashSet<i32> = cards
            .first()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let winning: Vec<i32> = winnum.intersection(&mynum).copied().collect();
        let mut points = 0;

        for w in 0..winning.len() {
            if w == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }

        sum += points;
    }

    Ok(sum)
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
    fn one() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process(input).unwrap(), 13);
    }
}
