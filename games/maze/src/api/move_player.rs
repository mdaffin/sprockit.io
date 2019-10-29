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
    Ok(HttpResponse::Ok().body("{}"))
}
#[cfg(test)]
mod tests {
    use super::{super::routes, SessionToken, Sessions};
    use crate::maze::tests::maze_from_slice_with_player_at;
    use crate::maze::{Direction, Tile};
    use actix_web::{test, web, App};
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    use std::sync::Mutex;

    mod when_player_moves_in {
        use super::*;
        lazy_static! {
            static ref BLOCKED_MAP: [Tile; 9] = [
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
        }

        #[test]
        fn open_direction_up_player_is_moved() {
            test_player_moves(Direction::Up, 1, 1, 1, 0, &[Tile::open(); 3 * 3]);
        }
        #[test]
        fn open_direction_down_player_is_moved() {
            test_player_moves(Direction::Down, 1, 1, 1, 2, &[Tile::open(); 3 * 3]);
        }
        #[test]
        fn open_direction_left_player_is_moved() {
            test_player_moves(Direction::Left, 1, 1, 0, 1, &[Tile::open(); 3 * 3]);
        }
        #[test]
        fn open_direction_right_player_is_moved() {
            test_player_moves(Direction::Right, 1, 1, 2, 1, &[Tile::open(); 3 * 3]);
        }

        #[test]
        fn blocked_direction_up_player_is_moved() {
            test_player_moves(Direction::Up, 1, 1, 1, 1, &BLOCKED_MAP[..]);
        }
        #[test]
        fn blocked_direction_down_player_is_moved() {
            test_player_moves(Direction::Down, 1, 1, 1, 1, &BLOCKED_MAP[..]);
        }
        #[test]
        fn blocked_direction_left_player_is_moved() {
            test_player_moves(Direction::Left, 1, 1, 1, 1, &BLOCKED_MAP[..]);
        }
        #[test]
        fn blocked_direction_right_player_is_moved() {
            test_player_moves(Direction::Right, 1, 1, 1, 1, &BLOCKED_MAP[..]);
        }

        #[test]
        fn edge_direction_up_player_is_moved() {
            test_player_moves(Direction::Up, 0, 0, 0, 0, &[Tile::open()]);
        }
        #[test]
        fn edge_direction_down_player_is_moved() {
            test_player_moves(Direction::Down, 0, 0, 0, 0, &[Tile::open()]);
        }
        #[test]
        fn edge_direction_left_player_is_moved() {
            test_player_moves(Direction::Left, 0, 0, 0, 0, &[Tile::open()]);
        }
        #[test]
        fn edge_direction_right_player_is_moved() {
            test_player_moves(Direction::Right, 0, 0, 0, 0, &[Tile::open()]);
        }

        fn test_player_moves(
            direction: Direction,
            start_x: usize,
            start_y: usize,
            end_x: usize,
            end_y: usize,
            map: &[Tile],
        ) {
            let (sessions, token, _) = run_test(direction, start_x, start_y, map);

            let player = {
                let sessions = sessions.lock().unwrap();
                (*sessions).get(&token).unwrap().player()
            };

            assert_eq!(player.x, end_x);
            assert_eq!(player.y, end_y);
        }

        fn run_test(
            direction: Direction,
            start_x: usize,
            start_y: usize,
            map: &[Tile],
        ) -> (Sessions, SessionToken, serde_json::Value) {
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

            let response: serde_json::Value = dbg!(test::read_response_json(&mut app, req));

            (sessions, token, response)
        }
    }
}
