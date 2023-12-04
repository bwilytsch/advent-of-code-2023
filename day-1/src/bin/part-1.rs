fn resolve(source: &str) -> u32 {
    let mut sum = 0;

    for line in source.lines() {
        let nums: Vec<u32> = line.chars().filter_map(|a| a.to_digit(10)).collect();

        let first = nums.first().unwrap_or(&0);
        let last = nums.last().unwrap_or(&0);

        let combined = format!("{}{}", first, last);

        let result = combined.parse::<u32>().unwrap();

        sum += result;
    }

    sum
}

fn main() {
    let input_one = include_str!("./input-1.txt");
    println!("Output One: {}", resolve(input_one));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let test_input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(resolve(test_input), 142);
    }
}
