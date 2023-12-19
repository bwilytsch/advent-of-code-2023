use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Tile {
    Empty,
    Galaxy,
}

type Grid = Vec<Tile>;

#[allow(dead_code)]
fn render_grid(grid: &Grid, rows: usize, columns: usize) {
    for i in 0..rows {
        for j in 0..columns {
            print!(
                "{}",
                match grid[i * columns + j] {
                    Tile::Empty => '.',
                    Tile::Galaxy => '#',
                }
            );
        }
        println!();
    }
}

fn get_row_by_index(grid: &Grid, idx: usize, width: usize) -> Result<Vec<Tile>, String> {
    if idx * width > grid.len() {
        return Err("Out of bounds".to_owned());
    }

    let mut result = vec![];
    let start = idx * width;

    for i in start..(start + width) {
        result.push(grid[i]);
    }

    Ok(result)
}

fn get_column_by_index(grid: &Grid, idx: usize, height: usize) -> Result<Vec<Tile>, String> {
    if idx * height > grid.len() {
        return Err("Out of bounds".to_owned());
    }

    let mut result = vec![];

    for i in 0..height {
        result.push(grid[idx + i * (height)]);
    }

    Ok(result)
}

fn process(input: &str) -> Result<i32, String> {
    let lines = input.lines().collect::<Vec<_>>();

    let mut rows = lines.len();
    let mut columns = lines[0].chars().collect::<Vec<_>>().len();

    // TODO: Do some .position() magic

    let mut grid = lines
        .iter()
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Galaxy,
                    _ => Tile::Empty,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Grid>();

    let mut expand_rows = vec![];
    let mut expand_columns = vec![];

    for i in 0..rows {
        if get_row_by_index(&grid, i, columns)
            .unwrap()
            .iter()
            .all(|t| *t == Tile::Empty)
        {
            expand_rows.push(i);
        }
    }

    for i in 0..columns {
        if get_column_by_index(&grid, i, rows)
            .unwrap()
            .iter()
            .all(|t| *t == Tile::Empty)
        {
            expand_columns.push(i);
        }
    }

    // Expand Universe
    for (i, row) in expand_rows.iter().enumerate() {
        grid.splice(
            (row + i) * columns..(row + i) * columns,
            vec![Tile::Empty; columns],
        );
    }

    rows += expand_rows.len();

    for (i, col) in expand_columns.iter().enumerate() {
        for j in 0..rows {
            // 2 + j * columns + j
            let new_index = col + i + j * (columns + i) + j;
            // println!("{}", new_index);
            grid.splice(
                // 2 + 0..Width * (Height + 2)
                new_index..new_index,
                vec![Tile::Empty],
            );
        }
    }

    columns += expand_columns.len();

    // render_grid(&grid, rows, columns);

    // Find pairs
    let galaxies = grid
        .iter()
        .enumerate()
        .filter_map(|(i, t)| {
            let x = i % columns;
            let y = i / columns;

            if t == &Tile::Galaxy {
                return Some(Point::new(x as i32, y as i32));
            }

            None
        })
        .collect::<Vec<_>>();

    let mut pairs: HashSet<Vec<Point>> = HashSet::new();

    for (i, a) in galaxies.iter().enumerate() {
        for j in 0..galaxies.len() {
            if i == j {
                continue;
            }

            if let Some(b) = galaxies.get(j) {
                let mut pair = vec![a.clone(), b.clone()];
                pair.sort();
                pairs.insert(pair);
            }
        }
    }

    // Get shortest paths between unique pairs
    // You can only walk up, down, left, right

    let mut sum = 0;

    for p in pairs {
        let a = p.get(0).unwrap();
        let b = p.get(1).unwrap();

        let delta_x = (a.x - b.x).abs();
        let delta_y = (a.y - b.y).abs();

        sum += delta_x + delta_y;
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
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(process(input).unwrap(), 374);
    }
}
