use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use failure::Fail;
use log::info;
use serde::Serialize;
use std::collections::HashMap;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Mutex;
use uuid::Uuid;

mod maze;
use maze::Maze;

type Sessions = web::Data<Mutex<HashMap<Uuid, Maze>>>;

#[derive(Debug, Serialize)]
struct ErrorResponse<'a> {
    error: &'a str,
    help: &'a str,
}

fn map(state: Sessions, req: HttpRequest) -> impl Responder {
    let token = match req.headers().get("x-token") {
        Some(t) => t,
        None => return HttpResponse::BadRequest().json(ErrorResponse {
                error: "missing session token",
                help: "The token can be set with the header `X-TOKEN`Session tokens are obtained by sending a post request to /start",
            }),
    };
    let token = match token.to_str() {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().json(ErrorResponse {
                error: &format!("token is not a valid utf-8 string: {}", e),
                help: "The token must be a valid utf-8 string. Session tokens are obtained by sending a post request to /start",
            }),
    };
    let token = match Uuid::parse_str(token) {
        Ok(t) => t,
        Err(e) => return HttpResponse::BadRequest().json(ErrorResponse {
                error: &format!("token is not a valid UUID: {}", e),
                help: "The token must be a valid utf-8 string. Session tokens are obtained by sending a post request to /start",
            }),
    };
    let sessions = state.lock().unwrap();
    let maze = match sessions.get(&token) {
        Some(map) => map,
        None => return HttpResponse::NotFound().json(ErrorResponse {
                error: &format!("session not found: {}", token),
                help: "No session was found for the given token. Session tokens are obtained by sending a post request to /start",
            }),
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
