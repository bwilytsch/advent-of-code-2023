use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Self { x, y }
    }
    fn inverse(&self) -> Point {
        Point::new(-self.x, -self.y)
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Tile {
    Ground,
    VerticalPipe,
    HorizontalPipe,
    StartingPoint,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Tile {
    fn from_char(c: char) -> Result<Tile, String> {
        match c {
            '.' => Ok(Tile::Ground),
            '|' => Ok(Tile::VerticalPipe),
            '-' => Ok(Tile::HorizontalPipe),
            'F' => Ok(Tile::SouthEast),
            'J' => Ok(Tile::NorthWest),
            'L' => Ok(Tile::NorthEast),
            '7' => Ok(Tile::SouthWest),
            'S' => Ok(Tile::StartingPoint),
            _ => Err(format!("Invalid character: {}", c)),
        }
    }
}

type Grid = Vec<Vec<Tile>>;
type TileMap = HashMap<Tile, (Point, Point)>;

fn render_path(grid: &Grid, visited: &HashSet<Point>) -> () {
    for (i, row) in grid.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            let p = Point::new(j as i32, i as i32);
            if visited.contains(&p) {
                match tile {
                    Tile::Ground => print!("."),
                    Tile::VerticalPipe => print!("|"),
                    Tile::HorizontalPipe => print!("-"),
                    Tile::StartingPoint => print!("S"),
                    Tile::NorthWest => print!("┘"),
                    Tile::NorthEast => print!("└"),
                    Tile::SouthWest => print!("┐"),
                    Tile::SouthEast => print!("┌"),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn walk(
    grid: &Grid,
    allowed_moves: &Vec<Point>,
    tile_map: &TileMap,
    visited: &mut HashSet<Point>,
    current_position: &Point,
) -> () {
    visited.insert(current_position.clone());

    let current_tile = grid
        .get(current_position.y as usize)
        .and_then(|row| row.get(current_position.x as usize));

    if current_tile.is_none() {
        return;
    }

    let (cur_entry, cur_exit) = tile_map.get(current_tile.unwrap()).unwrap();

    for step in allowed_moves {
        // Can I move out of this tile?
        if step != &cur_entry.inverse() && step != &cur_exit.inverse() {
            continue;
        }

        let check_position = Point::new(current_position.x + step.x, current_position.y + step.y);

        if !visited.contains(&check_position) {
            let tile = grid
                .get(check_position.y as usize)
                .and_then(|row| row.get(check_position.x as usize));

            if let Some(t) = tile {
                let (entry, exit) = tile_map.get(t).unwrap();

                // Can  I move into this tile?
                if entry == step || exit == step {
                    walk(grid, allowed_moves, tile_map, visited, &check_position);
                }
            }
        }
    }
}

fn process(input: &str) -> Result<usize, String> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut starting_point: Option<Point> = None;
    let mut grid: Grid = vec![];
    let mut tile_map: TileMap = HashMap::new();

    // The way you can enter this pipes
    tile_map.insert(Tile::Ground, (Point::new(0, 0), Point::new(0, 0)));
    tile_map.insert(Tile::StartingPoint, (Point::new(0, 0), Point::new(0, 0)));
    tile_map.insert(Tile::VerticalPipe, (Point::new(0, 1), Point::new(0, -1)));
    tile_map.insert(Tile::HorizontalPipe, (Point::new(1, 0), Point::new(-1, 0)));
    tile_map.insert(Tile::SouthEast, (Point::new(-1, 0), Point::new(0, -1)));
    tile_map.insert(Tile::SouthWest, (Point::new(1, 0), Point::new(0, -1)));
    tile_map.insert(Tile::NorthWest, (Point::new(1, 0), Point::new(0, 1)));
    tile_map.insert(Tile::NorthEast, (Point::new(-1, 0), Point::new(0, 1)));

    for line in lines {
        let row = line
            .chars()
            .filter_map(|c| Tile::from_char(c).ok())
            .collect();

        grid.push(row);
    }

    while starting_point.is_none() {
        for (y, row) in grid.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Tile::StartingPoint = tile {
                    starting_point = Some(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
    }

    if let Some(sp) = starting_point {
        println!("Starting point: {:?}", sp);
    }

    let sp = starting_point.unwrap();

    let moves = vec![
        Point::new(1, 0),  // Right
        Point::new(0, 1),  // Down
        Point::new(0, -1), // Up
        Point::new(-1, 0), // Left
    ];

    let mut max_visited = 0;

    for m in &moves {
        let mut visited = HashSet::new();
        visited.insert(sp.clone());
        let check_position = Point::new(sp.x + m.x, sp.y + m.y);
        walk(&grid, &moves, &tile_map, &mut visited, &check_position);

        render_path(&grid, &visited);

        if visited.len() > max_visited {
            max_visited = visited.len();
        }
    }

    Ok(max_visited / 2)
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
    fn simple_case() {
        let input = ".....
    .S-7.
    .|.|.
    .L-J.
    .....";
        assert_eq!(process(input).unwrap(), 4);
    }

    #[test]
    fn base_case() {
        let input = "..F7|
    .FJ||
    SJ.L7
    |F--J
    LJ...";
        assert_eq!(process(input).unwrap(), 8);
    }
}
