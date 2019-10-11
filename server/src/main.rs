use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/api/", web::get().to(greet))
            .route("/api/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:4000")
    .expect("Can not bind to port 4000")
    .run()
    .unwrap();
}
