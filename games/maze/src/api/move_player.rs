use super::{SessionToken, Sessions};
use crate::error::ServiceError;
use crate::maze::Direction;
use actix_web::{web, HttpResponse};

/// Moves a player in one direction if the path is not blocked.
pub fn move_player(
    direction: web::Path<Direction>,
    state: Sessions,
    token: SessionToken,
) -> Result<HttpResponse, ServiceError> {
    let mut sessions = state.lock().unwrap();
    let maze = sessions
        .get_mut(&token)
        .ok_or(ServiceError::SessionNotFound)?;

    maze.move_player(*direction);
    Ok(HttpResponse::Ok().body(""))
}
#[cfg(test)]
mod tests {
    use super::{super::routes, SessionToken, Sessions};
    use crate::maze::tests::maze_from_slice_with_player_at;
    use crate::maze::{Direction, Tile};
    use actix_web::{test, web, App};
    use std::collections::HashMap;
    use std::sync::Mutex;

    mod move_player_with {
        use super::*;

        #[test]
        fn open_direction_up() {
            test_open(Direction::Up, 1, 0);
        }
        #[test]
        fn open_direction_down() {
            test_open(Direction::Down, 1, 2);
        }
        #[test]
        fn open_direction_left() {
            test_open(Direction::Left, 0, 1);
        }
        #[test]
        fn open_direction_right() {
            test_open(Direction::Right, 2, 1);
        }

        #[test]
        fn blocked_direction_up() {
            test_blocked(Direction::Up);
        }
        #[test]
        fn blocked_direction_down() {
            test_blocked(Direction::Down);
        }
        #[test]
        fn blocked_direction_left() {
            test_blocked(Direction::Left);
        }
        #[test]
        fn blocked_direction_right() {
            test_blocked(Direction::Right);
        }

        #[test]
        fn edge_direction_up() {
            test_edge(Direction::Up);
        }
        #[test]
        fn edge_direction_down() {
            test_edge(Direction::Down);
        }
        #[test]
        fn edge_direction_left() {
            test_edge(Direction::Left);
        }
        #[test]
        fn edge_direction_right() {
            test_edge(Direction::Right);
        }

        fn test_open(direction: Direction, x: usize, y: usize) {
            test(direction, 1, 1, x, y, &[Tile::open(); 3 * 3])
        }

        fn test_blocked(direction: Direction) {
            let map = &[
                Tile::blocked(),
                Tile::blocked(),
                Tile::blocked(),
                Tile::blocked(),
                Tile::open(),
                Tile::blocked(),
                Tile::blocked(),
                Tile::blocked(),
                Tile::blocked(),
            ];
            test(direction, 1, 1, 1, 1, map)
        }

        fn test_edge(direction: Direction) {
            test(direction, 0, 0, 0, 0, &[Tile::open()])
        }

        fn test(
            direction: Direction,
            start_x: usize,
            start_y: usize,
            end_x: usize,
            end_y: usize,
            map: &[Tile],
        ) {
            let sessions: Sessions = web::Data::new(Mutex::new(HashMap::new()));
            let token = SessionToken::new();
            let maze = maze_from_slice_with_player_at(start_x, start_y, map);

            {
                let mut sessions = sessions.lock().unwrap();
                (*sessions).insert(token, maze);
            }

            let mut app =
                test::init_service(App::new().register_data(sessions.clone()).configure(routes));
            let req = test::TestRequest::post()
                .uri(&format!("/move/{}", direction))
                .header("X-TOKEN", token.to_string())
                .to_request();

            dbg!(test::call_service(&mut app, req));

            let player = {
                let sessions = sessions.lock().unwrap();
                (*sessions).get(&token).unwrap().player()
            };

            assert_eq!(player.x, end_x);
            assert_eq!(player.y, end_y);
        }
    }
}
