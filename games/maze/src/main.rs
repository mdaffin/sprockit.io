use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use failure::Fail;
use log::info;
use std::io;
use std::net::ToSocketAddrs;
mod maze;
use maze::Maze;

fn map(_: HttpRequest) -> impl Responder {
    let maze = Maze::new(10);
    HttpResponse::Ok().json(maze)
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
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/map", web::get().to(map))
    })
    .bind(addr)?
    .run()?;
    Ok(())
}
