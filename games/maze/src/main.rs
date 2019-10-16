use actix_web::{
    dev::Payload, middleware::Logger, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use derive_more::Display;
use failure::Fail;
use log::info;
use serde::Serialize;
use std::collections::HashMap;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Mutex;

mod error;
mod maze;

use error::ServiceError;
use maze::Maze;

type Sessions = web::Data<Mutex<HashMap<Token, Maze>>>;

#[derive(Debug, Serialize, Eq, PartialEq, Default, Display, Hash, Clone, Copy)]
struct Token(uuid::Uuid);

impl FromRequest for Token {
    type Error = ServiceError;
    type Future = Result<Self, ServiceError>;
    type Config = ();

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        let token = req
            .headers()
            .get("x-token")
            .ok_or(ServiceError::MissingSessionToken)?;
        let token = token.to_str().map_err(|_| ServiceError::InvalidTokenUTF8)?;
        Ok(Token(
            uuid::Uuid::parse_str(token).map_err(|_| ServiceError::InvalidTokenUUID)?,
        ))
    }
}

fn map(state: Sessions, token: Token) -> Result<HttpResponse, ServiceError> {
    let sessions = state.lock().unwrap();
    let maze = sessions.get(&token).ok_or(ServiceError::SessionNotFound)?;
    Ok(HttpResponse::Ok().json(maze))
}

fn start(state: Sessions, _: HttpRequest) -> impl Responder {
    #[derive(Debug, Serialize)]
    struct Response {
        token: Token,
    }

    let token = Token(uuid::Uuid::new_v4());
    let maze = Maze::new(10);

    {
        let mut sessions = state.lock().unwrap();
        (*sessions).insert(token, maze);
    }

    info!("New game started with token: {}", token);
    HttpResponse::Ok().json(Response { token })
}

fn main() {
    env_logger::init();

    if let Err(err) = run("localhost:4000") {
        for cause in Fail::iter_chain(&err) {
            println!("{}: {}", cause.name().unwrap_or("Error"), cause);
        }
    }
}

pub fn run(addr: impl ToSocketAddrs) -> Result<(), io::Error> {
    let sessions: Sessions = web::Data::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .register_data(sessions.clone())
            .wrap(Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .route("/map", web::get().to(map))
            .route("/start", web::post().to(start))
    })
    .bind(addr)?
    .run()?;
    Ok(())
}
