use super::{SessionToken, Sessions};
use crate::error::ServiceError;
use crate::maze::Direction;
use actix_web::{web, HttpResponse};

/// Moves a player in one direction if the path is not blocked and returns the tile_types of the directions the player can move to.
pub fn move_player(
    direction: web::Path<Direction>,
    state: Sessions,
    token: SessionToken,
) -> Result<HttpResponse, ServiceError> {
    let mut sessions = state.lock().unwrap();
    let session = sessions
        .get_mut(&token)
        .ok_or(ServiceError::SessionNotFound)?;

    session.mut_maze().move_player(*direction)?;
    Ok(HttpResponse::Ok().json(session.maze().neighbouring_tile_types()))
}

/// Returns the types for each tile that neighbours the players current position.
pub fn neighbouring_tile_types(
    state: Sessions,
    token: SessionToken,
) -> Result<HttpResponse, ServiceError> {
    let mut sessions = state.lock().unwrap();
    let session = sessions
        .get_mut(&token)
        .ok_or(ServiceError::SessionNotFound)?;
    Ok(HttpResponse::Ok().json(session.maze().neighbouring_tile_types()))
}

#[cfg(test)]
mod tests {
    use super::{
        super::{routes, Session},
        SessionToken, Sessions,
    };
    use crate::maze::tests::maze_from_slice_with_player_at;
    use crate::maze::{Direction, Position, Tile};
    use actix_web::{dev::ServiceResponse, test, web, App};
    use http::status::StatusCode;
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
            let (player, response) = setup_and_run(Direction::Up, 1, 1, &[Tile::open(); 3 * 3]);
            assert_eq!(player.x, 1);
            assert_eq!(player.y, 0);
            assert_eq!(response.status(), StatusCode::OK);
        }
        #[test]
        fn open_direction_down_player_is_moved() {
            let (player, response) = setup_and_run(Direction::Down, 1, 1, &[Tile::open(); 3 * 3]);
            assert_eq!(player.x, 1);
            assert_eq!(player.y, 2);
            assert_eq!(response.status(), StatusCode::OK);
        }
        #[test]
        fn open_direction_left_player_is_moved() {
            let (player, response) = setup_and_run(Direction::Left, 1, 1, &[Tile::open(); 3 * 3]);
            assert_eq!(player.x, 0);
            assert_eq!(player.y, 1);
            assert_eq!(response.status(), StatusCode::OK);
        }
        #[test]
        fn open_direction_right_player_is_moved() {
            let (player, response) = setup_and_run(Direction::Right, 1, 1, &[Tile::open(); 3 * 3]);
            assert_eq!(player.x, 2);
            assert_eq!(player.y, 1);
            assert_eq!(response.status(), StatusCode::OK);
        }

        #[test]
        fn blocked_direction_up_player_is_moved() {
            let (player, response) = setup_and_run(Direction::Up, 1, 1, &BLOCKED_MAP[..]);
            assert_eq!(player.x, 1);
            assert_eq!(player.y, 1);
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
        #[test]
        fn blocked_direction_down_player_is_moved() {
            let (player, response) = setup_and_run(Direction::Down, 1, 1, &BLOCKED_MAP[..]);
            assert_eq!(player.x, 1);
            assert_eq!(player.y, 1);
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
        #[test]
        fn blocked_direction_left_player_is_moved() {
            let (player, response) = setup_and_run(Direction::Left, 1, 1, &BLOCKED_MAP[..]);
            assert_eq!(player.x, 1);
            assert_eq!(player.y, 1);
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
        #[test]
        fn blocked_direction_right_player_is_moved() {
            let (player, response) = setup_and_run(Direction::Right, 1, 1, &BLOCKED_MAP[..]);
            assert_eq!(player.x, 1);
            assert_eq!(player.y, 1);
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }

        #[test]
        fn edge_direction_up_player_is_moved() {
            let (player, response) = setup_and_run(Direction::Up, 0, 0, &[Tile::open()]);
            assert_eq!(player.x, 0);
            assert_eq!(player.y, 0);
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
        #[test]
        fn edge_direction_down_player_is_moved() {
            let (player, response) = setup_and_run(Direction::Down, 0, 0, &[Tile::open()]);
            assert_eq!(player.x, 0);
            assert_eq!(player.y, 0);
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
        #[test]
        fn edge_direction_left_player_is_moved() {
            let (player, response) = setup_and_run(Direction::Left, 0, 0, &[Tile::open()]);
            assert_eq!(player.x, 0);
            assert_eq!(player.y, 0);
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }
        #[test]
        fn edge_direction_right_player_is_moved() {
            let (player, response) = setup_and_run(Direction::Right, 0, 0, &[Tile::open()]);
            assert_eq!(player.x, 0);
            assert_eq!(player.y, 0);
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }

        fn setup_and_run(
            direction: Direction,
            x: usize,
            y: usize,
            map: &[Tile],
        ) -> (Position, ServiceResponse) {
            let sessions: Sessions = web::Data::new(Mutex::new(HashMap::new()));
            let token = SessionToken::new();
            let maze = maze_from_slice_with_player_at(x, y, map);
            let session = Session { maze };

            {
                let mut sessions = sessions.lock().unwrap();
                (*sessions).insert(token, session);
            }

            let mut app =
                test::init_service(App::new().register_data(sessions.clone()).configure(routes));
            let req = test::TestRequest::post()
                .uri(&format!("/move/{}", direction))
                .header("X-TOKEN", token.to_string())
                .to_request();

            let response = dbg!(test::call_service(&mut app, req));

            let player = {
                let sessions = sessions.lock().unwrap();
                (*sessions).get(&token).unwrap().maze().player()
            };

            (player, response)
        }
    }
}
