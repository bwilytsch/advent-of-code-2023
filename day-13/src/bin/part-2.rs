#[derive(Debug)]
struct Pattern {
    values: Vec<char>,
    width: usize,
    height: usize,
}

impl From<&str> for Pattern {
    fn from(s: &str) -> Self {
        let width = s.lines().next().unwrap().len();
        Self {
            values: s.chars().filter(|c| *c != '\n').collect(),
            width,
            height: s.lines().count(),
        }
    }
}

impl Pattern {
    fn get_row_diff(&self, y1: usize, y2: usize) -> usize {
        let mut delta = 0;
        for x in 0..self.width {
            if self.values[y1 * self.width + x] != self.values[y2 * self.width + x] {
                delta += 1;
            }
        }
        delta
    }

    fn get_column_diff(&self, x1: usize, x2: usize) -> usize {
        let mut delta = 0;
        for y in 0..self.height {
            if self.values[y * self.width + x1] != self.values[y * self.width + x2] {
                delta += 1;
            }
        }
        delta
    }
}

fn process(input: &str) -> usize {
    let mut sum = 0;

    let patterns = input
        .split("\n\n")
        .map(|pattern| pattern.into())
        .collect::<Vec<Pattern>>();

    let offset = 1;

    // Find the mirrors
    for pattern in patterns {
        // Step through each row vertically and horizontally, skip first and last
        'rows: for i in 0..pattern.height - 1 {
            // A potential starting point is detected
            let mut diff = pattern.get_row_diff(i, i + 1);

            if diff <= 1 {
                let rows_to_check = i.min(pattern.height - i - 2);
                for j in 1..=rows_to_check {
                    diff += pattern.get_row_diff(i - j, i + j + 1);

                    if diff > 1 {
                        continue 'rows;
                    }
                }

                if diff == 0 {
                    continue 'rows;
                }

                sum += (i + offset) * 100;
            }
        }

        'columns: for i in 0..pattern.width - 1 {
            let mut diff = pattern.get_column_diff(i, i + 1);

            if diff <= 1 {
                let cols_to_check = i.min(pattern.width - i - 2);
                for j in 1..=cols_to_check {
                    diff += pattern.get_column_diff(i - j, i + j + 1);

                    if diff > 1 {
                        continue 'columns;
                    }
                }

                if diff == 0 {
                    continue 'columns;
                }

                sum += i + offset;
            }
        }
    }

    sum
}

fn main() {
    let input = include_str!("./input.txt");
    let now = std::time::Instant::now();
    let result = process(input);
    println!("Result: {} ({:?})", result, now.elapsed());
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
        assert_eq!(process(input), 400);
    }
}
