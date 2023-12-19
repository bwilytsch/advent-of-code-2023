use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Point {
    x: i128,
    y: i128,
}

impl Point {
    fn new(x: i128, y: i128) -> Self {
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

fn process(input: &str, expansion_factor: i128) -> Result<i128, String> {
    let lines = input.lines().collect::<Vec<_>>();

    let rows = lines.len();
    let columns = lines[0].chars().collect::<Vec<_>>().len();

    let grid = lines
        .iter()
        .flat_map(|line| {
            line.trim_start()
                .chars()
                .map(|c| match c {
                    '#' => Tile::Galaxy,
                    _ => Tile::Empty,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Grid>();

    // render_grid(&grid, rows, columns);

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

    // Find pairs
    let mut galaxies = grid
        .iter()
        .enumerate()
        .filter_map(|(i, t)| {
            let x = i % columns;
            let y = i / columns;

            if t == &Tile::Galaxy {
                return Some(Point::new(x as i128, y as i128));
            }

            None
        })
        .collect::<Vec<_>>();

    // The expansion needs to happen on the points only
    for col in expand_columns.iter().rev() {
        for g in galaxies.iter_mut() {
            if g.x > *col as i128 {
                // println!("col: {:?}, {:?}", col, g);
                g.x += expansion_factor - 1;
            }
        }
    }

    for row in expand_rows.iter().rev() {
        for g in galaxies.iter_mut() {
            if g.y > *row as i128 {
                g.y += expansion_factor - 1;
            }
        }
    }

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
    match process(input, 1000000) {
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
        assert_eq!(process(input, 2).unwrap(), 374);
    }

    #[test]
    fn base_case_10() {
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
        assert_eq!(process(input, 10).unwrap(), 1030);
    }
    #[test]
    fn base_case_100() {
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
        assert_eq!(process(input, 100).unwrap(), 8410);
    }
}
