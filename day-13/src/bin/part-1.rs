#[derive(Debug)]
struct Pattern {
    values: Vec<char>,
    width: usize,
    height: usize,
}

impl Pattern {
    fn get_row_at_index(&self, index: usize) -> String {
        let start = index * self.width;
        let end = start + self.width;
        self.values[start..end].into_iter().collect()
    }

    fn get_column_at_index(&self, index: usize) -> String {
        self.values
            .iter()
            .skip(index)
            .step_by(self.width)
            .map(|c| *c)
            .collect()
    }
}

fn process(input: &str) -> Result<usize, String> {
    let mut sum = 0;
    let mut patterns = vec![];
    let mut buffer: Vec<&str> = vec![];

    // Get all pattern blocks
    for line in input.lines() {
        if line.is_empty() {
            let width = buffer[0].len();
            patterns.push(Pattern {
                values: buffer.iter().flat_map(|s| s.chars()).collect(),
                width,
                height: buffer.len(),
            });
            buffer.clear();
            continue;
        }

        buffer.push(line);
    }

    // Collect last open block
    let width = buffer[0].len();
    patterns.push(Pattern {
        values: buffer.iter().flat_map(|s| s.chars()).collect(),
        width,
        height: buffer.len(),
    });

    let offset = 1;

    // Find the mirrors
    for pattern in patterns {
        // Step through each row vertically and horizontally, skip first and last
        'rows: for i in 0..pattern.height - 1 {
            // Check rows
            let mut current_row = pattern.get_row_at_index(i);
            let mut next_row = pattern.get_row_at_index(i + 1);

            // A potential starting point is detected
            if current_row == next_row {
                let start = i;
                let end = pattern.height - i - 2;

                let rows_to_check = start.min(end);

                for j in 1..=rows_to_check {
                    current_row = pattern.get_row_at_index(i - j);
                    next_row = pattern.get_row_at_index(i + j + 1);

                    if current_row != next_row {
                        continue 'rows;
                    }
                }

                sum += (i + offset) * 100;
            }
        }

        'columns: for i in 0..pattern.width - 1 {
            // Check columns
            let mut current_col = pattern.get_column_at_index(i);
            let mut next_col = pattern.get_column_at_index(i + 1);

            // A potential starting point is detected
            if current_col == next_col {
                let start = i;
                let end = pattern.width - i - 2;

                let cols_to_check = start.min(end);

                for j in 1..=cols_to_check {
                    current_col = pattern.get_column_at_index(i - j);
                    next_col = pattern.get_column_at_index(i + j + 1);

                    if current_col != next_col {
                        continue 'columns;
                    }
                }

                sum += i + offset;
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
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(process(input).unwrap(), 405);
    }
}
