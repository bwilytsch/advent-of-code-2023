use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
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

fn flood(grid: &Grid, loop_path: &HashSet<Point>) -> Vec<Point> {
    let mut output = vec![];

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let p = Point::new(j as i64, i as i64);
            let mut collisions = 0;

            if loop_path.contains(&p) {
                continue;
            }

            for pp in (0..j).rev() {
                let check = Point::new(pp as i64, i as i64);

                let tile = grid.get(i).and_then(|row| row.get(pp)).unwrap();

                if loop_path.contains(&check)
                    && tile != &Tile::HorizontalPipe
                    && tile != &Tile::SouthWest
                    && tile != &Tile::SouthEast
                    && tile != &Tile::StartingPoint
                {
                    collisions += 1;
                }
            }

            if collisions % 2 == 1 {
                output.push(p);
            }
        }
    }

    output
}

fn process(input: &str) -> Result<usize, String> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut starting_point: Option<Point> = None;
    let mut grid: Grid = vec![];
    let mut tile_map: TileMap = HashMap::new();

    // The way you can enter the pipes
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
                        x: x as i64,
                        y: y as i64,
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

    let mut loop_path = HashSet::new();

    // This runs the maze twice...
    for m in &moves {
        let mut visited = HashSet::new();
        visited.insert(sp.clone());
        let check_position = Point::new(sp.x + m.x, sp.y + m.y);
        walk(&grid, &moves, &tile_map, &mut visited, &check_position);

        if visited.len() > loop_path.len() {
            loop_path = visited.clone();
        }
    }

    let enclosed_tiles = flood(&grid, &loop_path);

    Ok(enclosed_tiles.len())
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
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(process(input).unwrap(), 10);
    }

    #[test]
    fn another_case() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(process(input).unwrap(), 8);
    }
}
