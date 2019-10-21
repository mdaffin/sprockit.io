use serde::ser::{SerializeSeq, Serializer};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Maze {
    player: Position,
    exit: Position,
    map: MazeMap,
}

#[derive(Debug, Clone)]
struct MazeMap {
    size: usize,
    map: Vec<Cell>,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub enum Cell {
    Blocked,
    Open,
}

impl Maze {
    pub fn new(size: usize) -> Self {
        let mut maze = Maze {
            player: Position { x: 0, y: 0 },
            exit: Position { x: size - 1, y: 0 },
            map: MazeMap {
                size,
                map: vec![Cell::Blocked; size * size],
            },
        };

        for x in 0..size {
            maze.set(x, 0, Cell::Open)
        }

        maze
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.map.set(x, y, cell);
    }
}

impl Serialize for MazeMap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.size))?;
        for i in 0..self.size {
            seq.serialize_element(&self.map[(i * self.size)..(i * self.size) + self.size])?;
        }
        seq.end()
    }
}

impl MazeMap {
    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.map[self.size * y + x] = cell;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    /// A new map gets created with a backing array of cells of size equal to a square of `size`
    /// sides.
    fn creating_maze_with_size() {
        for size in 1..100 {
            let maze = Maze::new(size);
            assert_eq!(maze.map.map.len(), size * size);
        }
    }

    #[test]
    /// The map maze should serialize to a 2d array instead of its internal representation.
    fn mazemap_serializes_to_a_2d_array() {
        let test_cases = [
            (3, r#"[["Open","Blocked","Blocked"],["Blocked","Open","Blocked"],["Blocked","Blocked","Blocked"]]"#),
            (2, r#"[["Open","Blocked"],["Blocked","Open"]]"#),
        ];
        for &(size, expected) in test_cases.into_iter() {
            let mut map = MazeMap {
                size,
                map: vec![Cell::Blocked; size * size],
            };

            map.set(0, 0, Cell::Open);
            map.set(1, 1, Cell::Open);

            let serialized = serde_json::to_string(&map).unwrap();
            assert_eq!(serialized.as_str(), expected);
        }
    }
}
