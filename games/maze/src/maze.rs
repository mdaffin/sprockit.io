use serde::ser::{SerializeSeq, Serializer};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Maze {
    player: Position,
    exit: Position,
    map: MazeMap,
}

#[derive(Debug)]
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
            exit: Position { x: size, y: 0 },
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
            seq.serialize_element(&self.map[i..i + self.size])?;
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

    #[test]
    fn creating_maze_with_size() {
        let size = 10;
        let maze = Maze::new(size);
        assert_eq!(maze.map.map.len(), size * size);
    }
}
