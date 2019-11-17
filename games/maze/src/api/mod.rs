use crate::error::ServiceError;
use crate::maze::Maze;
use actix_web::{dev::Payload, middleware::Logger, web, App, FromRequest, HttpRequest, HttpServer};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Mutex;

mod map;
mod move_player;
mod start;

type Sessions = web::Data<Mutex<HashMap<SessionToken, Session>>>;

pub struct Session {
    maze: Maze,
}

impl Session {
    pub fn new(size: usize) -> Self {
        Session {
            maze: Maze::new(size),
        }
    }

    pub fn maze(&self) -> &Maze {
        &self.maze
    }

    pub fn mut_maze(&mut self) -> &mut Maze {
        &mut self.maze
    }
}

/// A session token used to identify a currently running game. Users must supply this in the
/// X-TOKEN header and can obtain it from the /start endpoint.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Default, Display, Hash, Clone, Copy)]
pub struct SessionToken(uuid::Uuid);

/// Registers the routes for this API
fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/map").route(web::get().to(map::map)));
    cfg.service(web::resource("/move").route(web::get().to(move_player::neighbouring_tile_types)));
    cfg.service(web::resource("/start").route(web::post().to(start::start)));
    cfg.service(web::resource("/move/{direction}").route(web::post().to(move_player::move_player)));
}

/// Creates a new HTTP server on `addr` and runs it. This method blocks until the server is
/// shutdown.
pub fn run_server(addr: impl ToSocketAddrs) -> Result<(), io::Error> {
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
}
