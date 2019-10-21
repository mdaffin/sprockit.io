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
    map: Vec<Tile>,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tile {
    tile_type: TileType,
    visibility: TileVisibility,
}

#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
pub enum TileVisibility {
    Hidden,
    Revealed,
}

#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
pub enum TileType {
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
                map: vec![Tile::blocked(); size * size],
            },
        };

        for x in 0..size {
            maze.set(x, 0, Tile::open())
        }

        maze.reveal_around_player();

        maze
    }

    fn reveal_around_player(&mut self) {
        self.map.reveal(self.player.x, self.player.y);
        if self.player.x > 0 {
            self.map.reveal(self.player.x - 1, self.player.y);
        }
        if self.player.y > 0 {
            self.map.reveal(self.player.x, self.player.y - 1);
        }
        if self.player.x < self.map.size - 1 {
            self.map.reveal(self.player.x + 1, self.player.y);
        }
        if self.player.y < self.map.size - 1 {
            self.map.reveal(self.player.x, self.player.y + 1);
        }
    }

    fn set(&mut self, x: usize, y: usize, cell: Tile) {
        self.map.set(x, y, cell);
    }
}

impl Tile {
    pub fn open() -> Self {
        Tile {
            tile_type: TileType::Open,
            visibility: TileVisibility::Hidden,
        }
    }
    pub fn blocked() -> Self {
        Tile {
            tile_type: TileType::Blocked,
            visibility: TileVisibility::Hidden,
        }
    }

    pub fn reveal(&mut self) {
        self.visibility = TileVisibility::Revealed
    }

    pub fn is_visible(self) -> bool {
        self.visibility == TileVisibility::Revealed
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

impl Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.is_visible() {
            match self.tile_type {
                TileType::Open => serializer.serialize_str("open"),
                TileType::Blocked => serializer.serialize_str("blocked"),
            }
        } else {
            serializer.serialize_str("hidden")
        }
    }
}

impl MazeMap {
    fn to_index(&self, x: usize, y: usize) -> usize {
        self.size * y + x
    }

    fn reveal(&mut self, x: usize, y: usize) {
        let i = self.to_index(x, y);
        self.map[i].reveal();
    }

    fn set(&mut self, x: usize, y: usize, cell: Tile) {
        let i = self.to_index(x, y);
        self.map[i] = cell;
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
            (3, r#"[["open","hidden","blocked"],["hidden","blocked","blocked"],["blocked","blocked","blocked"]]"#),
            (2, r#"[["open","hidden"],["hidden","blocked"]]"#),
        ];
        for &(size, expected) in test_cases.into_iter() {
            let mut blocked = Tile::blocked();
            blocked.reveal();
            let mut open = Tile::open();
            open.reveal();
            let mut map = MazeMap {
                size,
                map: vec![blocked; size * size],
            };

            map.set(0, 0, open);
            map.set(1, 0, Tile::blocked());
            map.set(0, 1, Tile::open());

            let serialized = serde_json::to_string(&map).unwrap();
            assert_eq!(serialized.as_str(), expected);
        }
    }
}
