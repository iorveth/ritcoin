use std::env;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use crate::handlers::*;

const ADDRESS: &str = "127.0.0.1";

pub fn run() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/transaction/new").route(web::post().to(submit_tx)))
    })
    .bind((ADDRESS, port))?
    .run()
}