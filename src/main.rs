extern crate actix_web;
use actix_web::{http, server, App, Path, Responder};

fn index(info: Path<(String, String)>) -> impl Responder {
    format!("Hello from {} on the {} device!", info.0, info.1)
}

fn main() {
    server::new(
        || App::new()
            .route("/{os}/{device}/index.html", http::Method::GET, index))
        .bind("0.0.0.0:8080").unwrap()
        .run();
}
