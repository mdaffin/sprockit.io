use actix_web::{
    dev::Payload, middleware::Logger, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use derive_more::Display;
use failure::Fail;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Mutex;

mod error;
mod maze;

use error::ServiceError;
use maze::Maze;

type Sessions = web::Data<Mutex<HashMap<SessionToken, Maze>>>;

/// A session token used to identify a currently running game. Users must supply this in the
/// X-TOKEN header and can obtain it from the /start endpoint.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Default, Display, Hash, Clone, Copy)]
struct SessionToken(uuid::Uuid);

impl FromRequest for SessionToken {
    type Error = ServiceError;
    type Future = Result<Self, ServiceError>;
    type Config = ();

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        let token = req
            .headers()
            .get("x-token")
            .ok_or(ServiceError::MissingSessionToken)?;
        let token = token.to_str().map_err(|_| ServiceError::InvalidTokenUTF8)?;
        Ok(SessionToken(
            uuid::Uuid::parse_str(token).map_err(|_| ServiceError::InvalidTokenUUID)?,
        ))
    }
}

impl SessionToken {
    /// Creates a new randomly generated token.
    fn new() -> Self {
        SessionToken(uuid::Uuid::new_v4())
    }
}

/// The /map endpoint. Returns the map associated with the session token passed into the request.
fn map(state: Sessions, token: SessionToken) -> Result<HttpResponse, ServiceError> {
    let sessions = state.lock().unwrap();
    let maze = sessions.get(&token).ok_or(ServiceError::SessionNotFound)?;
    Ok(HttpResponse::Ok().json(maze))
}

/// The /start endpoint. Creates a new game session and returns the token used to idenfiy this
/// session.
fn start(state: Sessions) -> impl Responder {
    #[derive(Debug, Serialize)]
    struct Response {
        token: SessionToken,
    }

    let token = SessionToken::new();
    let maze = Maze::new(10);

    {
        let mut sessions = state.lock().unwrap();
        (*sessions).insert(token, maze);
    }

    info!("New game started with token: {}", token);
    HttpResponse::Ok().json(Response { token })
}

/// Creates a new HTTP server on `addr` and runs it. This method blocks until the server is
/// shutdown.
fn run_server(addr: impl ToSocketAddrs) -> Result<(), io::Error> {
    let sessions: Sessions = web::Data::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .register_data(sessions.clone())
            .data(web::JsonConfig::default().limit(4096))
            .configure(routes)
    })
    .bind(addr)?
    .run()?;
    Ok(())
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/map").route(web::get().to(map)));
    cfg.service(web::resource("/start").route(web::post().to(start)));
}

fn main() {
    env_logger::init();

    if let Err(err) = run_server("localhost:4000") {
        for cause in Fail::iter_chain(&err) {
            println!("{}: {}", cause.name().unwrap_or("Error"), cause);
        }
    }
}

#[cfg(test)]
mod tests {
    mod session_token {
        use super::super::{ServiceError, SessionToken};
        use actix_web::{
            http::header::HeaderValue,
            test::{block_on, TestRequest},
            FromRequest,
        };

        #[test]
        /// The session token should be extracted from the X-TOKEN header if present.
        fn extracted_from_header() {
            let uuid = uuid::Uuid::new_v4();
            let (req, mut payload) =
                TestRequest::with_header("X-TOKEN", format!("{}", uuid)).to_http_parts();

            let token = block_on(SessionToken::from_request(&req, &mut payload));

            assert_eq!(token, Ok(SessionToken(uuid)));
        }

        #[test]
        /// If the token is missing it should return an error
        fn missing_token() {
            let (req, mut payload) = TestRequest::default().to_http_parts();

            let token = block_on(SessionToken::from_request(&req, &mut payload));

            assert_eq!(token, Err(ServiceError::MissingSessionToken));
        }

        #[test]
        /// If the token is not valid utf8 it should return an error
        fn none_utf8_token() {
            let (req, mut payload) =
                TestRequest::with_header("X-TOKEN", HeaderValue::from_bytes(&[245]).unwrap())
                    .to_http_parts();

            let token = block_on(SessionToken::from_request(&req, &mut payload));

            assert_eq!(token, Err(ServiceError::InvalidTokenUTF8));
        }

        #[test]
        /// If the token is not a valid uuid it should return an error
        fn none_uuid_token() {
            let (req, mut payload) =
                TestRequest::with_header("X-TOKEN", "not-a-uuid").to_http_parts();

            let token = block_on(SessionToken::from_request(&req, &mut payload));

            assert_eq!(token, Err(ServiceError::InvalidTokenUUID));
        }
    }

    mod start {
        use super::super::{routes, SessionToken, Sessions};
        use actix_web::{test, web, App};
        use serde::Deserialize;
        use std::collections::HashMap;
        use std::sync::Mutex;

        #[test]
        /// Starting a new session adds the session to the store and returns the corrisponing token
        fn new_session_stored_and_token_returned() {
            #[derive(Debug, Deserialize)]
            struct Response {
                token: SessionToken,
            }

            let sessions: Sessions = web::Data::new(Mutex::new(HashMap::new()));
            let mut app =
                test::init_service(App::new().register_data(sessions.clone()).configure(routes));
            let req = test::TestRequest::post().uri("/start").to_request();

            let response: HashMap<String, SessionToken> = test::read_response_json(&mut app, req);

            assert!(response.get("token").is_some());
            assert_eq!(
                response.get("token"),
                sessions.lock().unwrap().keys().next()
            );
        }
    }
}
