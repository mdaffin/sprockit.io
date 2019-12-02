use super::{SessionToken, Sessions};
use crate::error::ServiceError;
use actix_web::{web, HttpResponse};
use maze::Direction;

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
