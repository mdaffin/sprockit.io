use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Maze {
    player: Position,
    exit: Position,
    size: usize,
    grid: Vec<Cell>,
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
            size,
            grid: vec![Cell::Blocked; size * size],
        };

        for x in 0..size {
            maze.set(x, 0, Cell::Open)
        }

        maze
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        self.grid[self.size * y + x] = cell;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_maze_with_size() {
        let size = 10;
        let maze = Maze::new(size);
        assert_eq!(maze.grid.len(), size * size);
    }
}
