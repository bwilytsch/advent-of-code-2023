fn process(input: &str) -> Result<i32, String> {
    Ok(0)
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
        let input = "";
        assert_eq!(process(input).unwrap(), 0);
    }
}
