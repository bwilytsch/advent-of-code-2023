#[derive(Debug)]
struct Gear {
    value: u32,
    range: (usize, usize),
}

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

fn get_neightbors(x: i32, y: i32, grid: &Grid) -> Vec<usize> {
    let mut neighbors: Vec<usize> = vec![];

    for i in -1..=1 {
        for j in -1..=1 {
            let new_x = x + i;
            let new_y = y + j;

            let new_idx = get_index(new_x, new_y, grid.rows as i32);
            let len = grid.cells.len() as i32;

            if 0 <= new_idx && new_idx < len {
                neighbors.push(new_idx as usize);
            }
        }
    }

    neighbors
}

fn process(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut gears: Vec<Gear> = vec![];

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

            gears.push(Gear {
                value: number.parse::<u32>().unwrap(),
                range: (pointer, pointer + cursor),
            });

            pointer += cursor;
            continue;
        }

        pointer += 1;
    }

    // Find all * and see if there are gears connected
    for (idx, c) in reader.iter().enumerate() {
        if *c == '*' {
            let coord = get_coordinates(&idx, &grid.columns);
            let neighbors_indecies = get_neightbors(coord.x as i32, coord.y as i32, &grid);
            let adjacent_gears = gears
                .iter()
                .filter(|g| {
                    let (start, end) = g.range;

                    for i in start..end {
                        if neighbors_indecies.contains(&i) {
                            return true;
                        }
                    }

                    false
                })
                .collect::<Vec<_>>();

            if adjacent_gears.len() > 1 {
                let total = adjacent_gears[0].value * adjacent_gears[1].value;
                sum += total;
            }
        }
    }

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
        assert_eq!(process(input), 467835);
    }
}
