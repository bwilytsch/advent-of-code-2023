struct Coord {
    x: usize,
    y: usize,
}

struct Grid {
    cells: Vec<char>,
    rows: usize,
    columns: usize,
}

fn get_coordinates(idx: &usize, columns: &usize) -> Coord {
    let x = idx % columns;
    let y = idx / columns;

    Coord { x, y }
}

fn get_index(x: i32, y: i32, rows: i32) -> i32 {
    y * rows + x
}

fn get_neightbors(x: i32, y: i32, grid: &Grid) -> Vec<char> {
    let mut neighbors: Vec<char> = vec![];

    for i in -1..=1 {
        for j in -1..=1 {
            let new_x = x + i;
            let new_y = y + j;

            let new_idx = get_index(new_x, new_y, grid.rows as i32);
            let len = grid.cells.len() as i32;

            if 0 <= new_idx && new_idx < len {
                neighbors.push(grid.cells[new_idx as usize]);
            }
        }
    }

    neighbors
}

// Moore Neighbour algorithm
fn is_valid(idx: usize, len: usize, grid: &Grid) -> bool {
    let mut symbols = 0;
    let allowed_symbols: Vec<char> = "*#+$/@=%&-".chars().collect::<Vec<_>>();

    for i in 0..len {
        let coord = get_coordinates(&(idx + i), &grid.columns);
        let neighbors = get_neightbors(coord.x as i32, coord.y as i32, &grid);

        // println!("{} {:?}", grid.cells[idx + i], neighbors);

        symbols += neighbors
            .iter()
            .filter(|c| allowed_symbols.contains(c))
            .collect::<Vec<_>>()
            .len();
    }

    symbols > 0
}

fn process(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut nums: Vec<u32> = vec![];

    let reader = input.chars().filter(|c| *c != '\n').collect::<Vec<char>>();
    let lines = input.lines().collect::<Vec<&str>>();

    let grid = Grid {
        cells: reader.clone(),
        rows: lines.len(),
        columns: lines[0].len(),
    };

    let mut pointer = 0;

    while pointer < reader.len() {
        let c = reader[pointer];

        if c.is_numeric() {
            let mut next_char = c;
            let mut cursor = 0;
            let mut number = String::new();

            while next_char.is_numeric() {
                number.push(next_char);
                cursor += 1;

                if pointer + cursor >= reader.len() {
                    break;
                }

                next_char = reader[pointer + cursor];
            }

            // Check if number has an adjecent symbol
            if is_valid(pointer, cursor, &grid) {
                nums.push(number.parse::<u32>().unwrap());
            }

            pointer += cursor;
            continue;
        }

        pointer += 1;
    }

    let total: u32 = nums.iter().sum();
    sum += total;
    sum
}

fn main() {
    let input_file = include_str!("./input-1.txt");
    println!("{}", process(input_file));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(process(input), 4361);
    }
}
