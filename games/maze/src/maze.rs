use crate::error::ServiceError;
use rand::seq::SliceRandom;
use rand::thread_rng;
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

#[derive(Debug, Clone)]
struct MazeGenerationTile {
    position: Position,
    link: Position,
    tile: Tile,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct NeighbouringTileTypes {
    left: TileType,
    right: TileType,
    up: TileType,
    down: TileType,
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
    Neither,
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
        fn find(
            map: &Vec<Vec<MazeGenerationTile>>,
            (p, q): (Position, Position),
        ) -> (Position, Position) {
            let cell_p = map[p.y][p.x].link;
            let cell_q = map[q.y][q.x].link;
            if p != cell_p || q != cell_q {
                find(map, (cell_p, cell_q))
            } else {
                (cell_p, cell_q)
            }
        }

        let mut gen_map: Vec<Vec<MazeGenerationTile>> = vec![];

        for i in 0..size {
            let mut gen_row: Vec<_> = vec![];
            for j in 0..size {
                let pos = Position { x: j, y: i };
                let (j_is_even, i_is_even) = (j % 2 == 0, i % 2 == 0);

                // TODO: match?
                if !j_is_even && !i_is_even {
                    gen_row.push(MazeGenerationTile {
                        position: pos,
                        ..MazeGenerationTile::blocked(pos)
                    });
                } else if j_is_even && i_is_even {
                    gen_row.push(MazeGenerationTile {
                        position: pos,
                        ..MazeGenerationTile::open(pos)
                    });
                } else if (!j_is_even && i_is_even) || (j_is_even && !i_is_even) {
                    gen_row.push(MazeGenerationTile {
                        position: pos,
                        ..MazeGenerationTile::neither(pos)
                    });
                }
            }
            gen_map.push(gen_row);
        }

        let neither_map = gen_map.to_vec();
        let mut neither_map = neither_map
            .iter()
            .flatten()
            .filter(|x| match x.tile.tile_type {
                TileType::Neither => true,
                _ => false,
            })
            .collect::<Vec<_>>();

        neither_map.shuffle(&mut thread_rng());

        for i in neither_map {
            let find_map = |x| find(&gen_map, x);
            let pos = i.position;

            let (p, q) = match pos.y & 1 == 0 {
                true => find_map((
                    Position {
                        x: pos.x + 1,
                        y: pos.y,
                    },
                    Position {
                        x: pos.x - 1,
                        y: pos.y,
                    },
                )),
                false => find_map((
                    Position {
                        x: pos.x,
                        y: pos.y - 1,
                    },
                    Position {
                        x: pos.x,
                        y: pos.y + 1,
                    },
                )),
            };
            if p != q {
                gen_map[pos.y][pos.x].tile = Tile::open();
                gen_map[p.y][p.x].link = q;
            } else {
                gen_map[pos.y][pos.x].tile = Tile::blocked();
            }
        }

        let mut maze = Maze {
            player: Position { x: 0, y: 0 },
            exit: Position {
                x: size - 1,
                y: size - 1,
            },
            size,
            map: gen_map.iter().flatten().map(|x| x.tile).collect::<Vec<_>>(),
        };

        //maze.reveal_around_player();
        maze.reveal_all();
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

    fn tile_at(&self, x: usize, y: usize) -> Tile {
        self.map[self.to_index(x, y)]
    }

    fn tile_type_at(&self, x: i32, y: i32) -> TileType {
        if x < 0 || y < 0 || x >= self.size as i32 || y >= self.size as i32 {
            TileType::Blocked
        } else {
            self.tile_at(x as usize, y as usize).tile_type
        }
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
            || self.tile_at(x as usize, y as usize).tile_type == TileType::Blocked
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

    pub fn neighbouring_tile_types(&self) -> NeighbouringTileTypes {
        let player_x = self.player.x as i32;
        let player_y = self.player.y as i32;

        NeighbouringTileTypes {
            left: self.tile_type_at(player_x - 1, player_y),
            right: self.tile_type_at(player_x + 1, player_y),
            up: self.tile_type_at(player_x, player_y - 1),
            down: self.tile_type_at(player_x, player_y + 1),
        }
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

    fn reveal_all(&mut self) {
        for i in 0..self.size {
            for j in 0..self.size {
                self.reveal(i, j);
            }
        }
    }
}

impl MazeGenerationTile {
    fn open(pos: Position) -> Self {
        MazeGenerationTile {
            position: pos,
            link: pos,
            tile: Tile {
                tile_type: TileType::Open,
                visibility: TileVisibility::Hidden,
            },
        }
    }

    fn blocked(pos: Position) -> Self {
        MazeGenerationTile {
            position: pos,
            link: pos,
            tile: Tile {
                tile_type: TileType::Blocked,
                visibility: TileVisibility::Hidden,
            },
        }
    }

    fn neither(pos: Position) -> Self {
        MazeGenerationTile {
            position: pos,
            link: pos,
            tile: Tile {
                tile_type: TileType::Neither,
                visibility: TileVisibility::Hidden,
            },
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
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
                _ => panic!("Neither open nor blocked"),
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

#[cfg(test)]
mod neighbouring_tile_types {
    use super::*;
    use crate::maze::tests::maze_from_slice_with_player_at;

    fn neighbouring_tile_types_test_setup(
        size: usize,
        player_position: Position,
    ) -> NeighbouringTileTypes {
        let maze = maze_from_slice_with_player_at(
            player_position.x,
            player_position.y,
            &vec![Tile::open(); size * size],
        );
        maze.neighbouring_tile_types()
    }

    #[test]
    /// Checks that negative coordinates given to the neighbouring_tile_types function actually return blocked
    fn when_in_upper_left_corner_up_and_left_are_blocked() {
        let tile_types_actual = neighbouring_tile_types_test_setup(2, Position { x: 0, y: 0 });
        let tile_types_should_be = NeighbouringTileTypes {
            left: TileType::Blocked,
            right: TileType::Open,
            up: TileType::Blocked,
            down: TileType::Open,
        };
        assert_eq!(tile_types_actual, tile_types_should_be);
    }

    #[test]
    /// Checks that the neighbouring_tile_types function returns simply the map given no borders
    fn when_in_middle_all_open() {
        let tile_types_actual = neighbouring_tile_types_test_setup(3, Position { x: 1, y: 1 });
        let tile_types_should_be = NeighbouringTileTypes {
            left: TileType::Open,
            right: TileType::Open,
            up: TileType::Open,
            down: TileType::Open,
        };
        assert_eq!(tile_types_actual, tile_types_should_be);
    }

    #[test]
    /// Checks that coordinates given to the neighbouring_tile_types function that exceeds the size actually return blocked
    fn when_in_bottom_right_corner_down_and_right_are_blocked() {
        let tile_types_actual = neighbouring_tile_types_test_setup(100, Position { x: 99, y: 99 });
        let tile_types_should_be = NeighbouringTileTypes {
            left: TileType::Open,
            right: TileType::Blocked,
            up: TileType::Open,
            down: TileType::Blocked,
        };
        assert_eq!(tile_types_actual, tile_types_should_be);
    }
}
