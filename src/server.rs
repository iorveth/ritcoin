use crate::handlers::*;
use crate::*;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use std::env;
pub const ADDRESS: &str = "127.0.0.1";
pub const BROADCAST_RESOURCE: &str = "/transaction/new";
pub const PENDINGS_RESOURCE: &str = "/transaction/pendings";


pub fn run(ritcoin_state: Arc<RitCoinState>) -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(move || {
        App::new()
            .data(ritcoin_state.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource(BROADCAST_RESOURCE).route(web::post().to(handle_submit_tx)))
            .service(web::resource(PENDINGS_RESOURCE).route(web::post().to(handle_pendings)))
    })
    .bind((ADDRESS, port))?
    .run()
}
