use super::{SessionToken, Sessions};
use crate::error::ServiceError;
use actix_web::HttpResponse;

/// The /map endpoint. Returns the map associated with the session token passed into the request.
pub fn map(state: Sessions, token: SessionToken) -> Result<HttpResponse, ServiceError> {
    let sessions = state.lock().unwrap();
    let maze = sessions.get(&token).ok_or(ServiceError::SessionNotFound)?;
    Ok(HttpResponse::Ok().json(maze))
}

#[cfg(test)]
mod tests {
    use super::{super::routes, SessionToken, Sessions};
    use crate::maze::Maze;
    use actix_web::{http::StatusCode, test, web, App};
    use bytes::Bytes;
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[test]
    /// Return the map for the game session requested
    fn returns_maze_associated_with_token() {
        let sessions: Sessions = web::Data::new(Mutex::new(HashMap::new()));
        let token = SessionToken::new();
        let maze = Maze::new(10);

        {
            let mut sessions = sessions.lock().unwrap();
            (*sessions).insert(token, maze.clone());
        }

        let mut app =
            test::init_service(App::new().register_data(sessions.clone()).configure(routes));
        let req = test::TestRequest::get()
            .uri("/map")
            .header("X-TOKEN", token.to_string())
            .to_request();

        let response = test::read_response(&mut app, req);

        assert_eq!(response, Bytes::from(serde_json::to_string(&maze).unwrap()));
    }

    #[test]
    /// Return a 404 if the session for the given token does not exist
    fn returns_404_with_valid_token() {
        let sessions: Sessions = web::Data::new(Mutex::new(HashMap::new()));

        let mut app = test::init_service(App::new().register_data(sessions).configure(routes));
        let req = test::TestRequest::get()
            .uri("/map")
            .header("X-TOKEN", SessionToken::new().to_string())
            .to_request();

        let response = test::call_service(&mut app, req);

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
