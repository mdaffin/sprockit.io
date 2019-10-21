use super::{SessionToken, Sessions};
use crate::error::ServiceError;
use actix_web::HttpResponse;

/// The /map endpoint. Returns the map associated with the session token passed into the request.
pub fn map(state: Sessions, token: SessionToken) -> Result<HttpResponse, ServiceError> {
    let sessions = state.lock().unwrap();
    let maze = sessions.get(&token).ok_or(ServiceError::SessionNotFound)?;
    Ok(HttpResponse::Ok().json(maze))
}
