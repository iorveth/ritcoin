use crate::handlers::*;
use crate::*;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use std::env;
pub const ADDRESS: &str = "127.0.0.1";
pub const BROADCAST_RESOURCE: &str = "/transaction/new";

pub fn run(ritcoin_state: Arc<RitCoinState>) -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(move || {
        App::new()
            .data(ritcoin_state.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource(BROADCAST_RESOURCE).route(web::post().to(submit_tx)))
    })
    .bind((ADDRESS, port))?
    .run()
}
