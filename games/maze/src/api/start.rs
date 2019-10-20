use actix_web::HttpResponse;
use log::info;
use serde::{Deserialize, Serialize};

use super::{SessionToken, Sessions};
use crate::maze::Maze;

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    token: SessionToken,
}

/// The /start endpoint. Creates a new game session and returns the token used to idenfiy this
/// session.
pub fn start(state: Sessions) -> HttpResponse {
    let token = SessionToken::new();
    let maze = Maze::new(10);

    {
        let mut sessions = state.lock().unwrap();
        (*sessions).insert(token, maze);
    }

    info!("New game started with token: {}", token);
    HttpResponse::Ok().json(Response { token })
}

#[cfg(test)]
mod tests {
    use super::{super::routes, Response, Sessions};
    use actix_web::{test, web, App};
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[test]
    /// Starting a new session adds the session to the store and returns the corresponding token
    fn new_session_stored_and_token_returned() {
        let sessions: Sessions = web::Data::new(Mutex::new(HashMap::new()));
        let mut app =
            test::init_service(App::new().register_data(sessions.clone()).configure(routes));
        let req = test::TestRequest::post().uri("/start").to_request();

        let response: Response = test::read_response_json(&mut app, req);

        assert_eq!(
            Some(&response.token),
            sessions.lock().unwrap().keys().next()
        );
    }
}
