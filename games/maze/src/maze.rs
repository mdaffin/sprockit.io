use crate::error::ServiceError;
use serde::ser::{SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Maze {
    player: Position,
    exit: Position,
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
enum TileVisibility {
    Hidden,
    Revealed,
}

#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
enum TileType {
    Blocked,
    Open,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub enum Direction {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}

impl Maze {
    pub fn new(size: usize) -> Self {
        let mut maze = Maze {
            player: Position { x: 0, y: 0 },
            exit: Position {
                x: size - 1,
                y: size - 1,
            },
            size,
            map: vec![Tile::blocked(); size * size],
        };

        for i in 0..size {
            maze.set(i, 0, Tile::open());
            maze.set(size - 1, i, Tile::open());
        }

        maze.reveal_around_player();

        maze
    }

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

    fn get(&mut self, x: usize, y: usize) -> Tile {
        self.map[self.to_index(x, y)]
    }

    pub fn move_player(&mut self, direction: Direction) -> Result<(), ServiceError> {
        use Direction::*;

        let (x, y) = match direction {
            Up => (self.player.x as i32, self.player.y as i32 - 1),
            Down => (self.player.x as i32, self.player.y as i32 + 1),
            Left => (self.player.x as i32 - 1, self.player.y as i32),
            Right => (self.player.x as i32 + 1, self.player.y as i32),
        };

        if x < 0
            || y < 0
            || (x as usize) >= self.size
            || (y as usize) >= self.size
            || self.get(x as usize, y as usize).tile_type == TileType::Blocked
        {
            return Err(ServiceError::DirectionBlocked);
        }

        self.player = Position {
            x: x as usize,
            y: y as usize,
        };

        self.reveal_around_player();
        Ok(())
    }

    #[cfg(test)]
    pub fn player(&self) -> Position {
        self.player
    }

    fn reveal_around_player(&mut self) {
        self.reveal(self.player.x, self.player.y);
        if self.player.x > 0 {
            self.reveal(self.player.x - 1, self.player.y);
        }
        if self.player.y > 0 {
            self.reveal(self.player.x, self.player.y - 1);
        }
        if self.player.x < self.size - 1 {
            self.reveal(self.player.x + 1, self.player.y);
        }
        if self.player.y < self.size - 1 {
            self.reveal(self.player.x, self.player.y + 1);
        }
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

    pub fn is_revealed(self) -> bool {
        self.visibility == TileVisibility::Revealed
    }
}

impl Serialize for Maze {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // To serialise a single row without copying all the elements into a new array.
        struct Row<'a> {
            row_index: usize,
            player: &'a Position,
            exit: &'a Position,
            elements: &'a [Tile],
        }

        impl<'a> Serialize for Row<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut seq = serializer.serialize_seq(Some(self.elements.len()))?;
                for x in 0..self.elements.len() {
                    if self.player.x == x && self.player.y == self.row_index {
                        seq.serialize_element("player")?;
                    } else if self.exit.x == x && self.exit.y == self.row_index {
                        seq.serialize_element("exit")?;
                    } else {
                        seq.serialize_element(&self.elements[x])?;
                    }
                }
                seq.end()
            }
        }

        let mut seq = serializer.serialize_seq(Some(self.size))?;
        for y in 0..self.size {
            seq.serialize_element(&Row {
                row_index: y,
                player: &self.player,
                exit: &self.exit,
                elements: &self.map[(y * self.size)..(y * self.size) + self.size],
            })?;
        }
        seq.end()
    }
}

impl Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.is_revealed() {
            match self.tile_type {
                TileType::Open => serializer.serialize_str("open"),
                TileType::Blocked => serializer.serialize_str("blocked"),
            }
        } else {
            serializer.serialize_str("hidden")
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Direction::*;
        write!(
            f,
            "{}",
            match self {
                Up => "up",
                Down => "down",
                Left => "left",
                Right => "right",
            }
        )
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde_json;

    pub fn maze_from_slice_with_player_at(x: usize, y: usize, map: &[Tile]) -> Maze {
        let size = (map.len() as f64).sqrt() as usize;
        assert_eq!(map.len(), size * size);
        let mut maze = Maze {
            player: Position { x, y },
            exit: Position {
                x: size - 1,
                y: size - 1,
            },
            size,
            map: Vec::from(map),
        };

        maze.reveal_around_player();
        maze
    }

    #[test]
    /// A new map gets created with a backing array of cells of size equal to a square of `size`
    /// sides.
    fn creating_maze_with_size() {
        for size in 1..100 {
            let maze = Maze::new(size);
            assert_eq!(maze.map.len(), size * size);
        }
    }

    #[test]
    /// The map maze should serialize to a 2d array instead of its internal representation.
    fn mazemap_serializes_to_a_2d_array() {
        let test_cases = [
            (3, r#"[["player","hidden","blocked"],["hidden","blocked","blocked"],["blocked","blocked","exit"]]"#),
            (2, r#"[["player","hidden"],["hidden","exit"]]"#),
        ];
        for &(size, expected) in test_cases.into_iter() {
            let mut blocked = Tile::blocked();
            blocked.reveal();
            let mut open = Tile::open();
            open.reveal();
            let mut map = maze_from_slice_with_player_at(0, 0, &vec![blocked; size * size]);

            map.set(0, 0, open);
            map.set(1, 0, Tile::blocked());
            map.set(0, 1, Tile::open());

            let serialized = serde_json::to_string(&map).unwrap();
            assert_eq!(serialized.as_str(), expected);
        }
    }
}
