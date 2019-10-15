use actix_http::error::ResponseError;
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use failure::Fail;
use log::info;
use serde::Serialize;
use std::collections::HashMap;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Mutex;
use uuid::Uuid;

mod error;
mod maze;

use error::ServiceError;
use maze::Maze;

type Sessions = web::Data<Mutex<HashMap<Uuid, Maze>>>;

fn map(state: Sessions, req: HttpRequest) -> impl Responder {
    let token = match req.headers().get("x-token") {
        Some(t) => t,
        None => return ServiceError::MissingSessionToken.error_response(),
    };
    let token = match token.to_str() {
        Ok(t) => t,
        Err(_) => return ServiceError::InvalidTokenUTF8.error_response(),
    };
    let token = match Uuid::parse_str(token) {
        Ok(t) => t,
        Err(_) => return ServiceError::InvalidTokenUUID.error_response(),
    };
    let sessions = state.lock().unwrap();
    let maze = match sessions.get(&token) {
        Some(map) => map,
        None => return ServiceError::SessionNotFound.error_response(),
    };

    HttpResponse::Ok().json(maze)
}

fn start(state: Sessions, _: HttpRequest) -> impl Responder {
    #[derive(Debug, Serialize)]
    struct Response {
        token: Uuid,
    }

    let token = Uuid::new_v4();
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
