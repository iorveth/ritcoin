use crate::handlers::*;
use crate::*;
use actix_web::{middleware, web, App, HttpServer};
use std::env;

pub const DEFAULT_ADDRESS: &str = "0.0.0.0:3000";
pub const DEFAULT_PORT: &str = "3000";
pub const DEFAULT_IP: &str = "0.0.0.0";
pub const BROADCAST_RESOURCE: &str = "/transaction/new";
pub const PENDINGS_RESOURCE: &str = "/transaction/pendings";
pub const CHAIN_RESOURCE: &str = "/chain";
pub const NODES_RESOURCE: &str = "/nodes";
pub const CHAIN_LENGTH_RESOURCE: &str = "/chain/length";

pub fn run(ritcoin_state: Arc<RitCoinState>) -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| DEFAULT_PORT.to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(move || {
        App::new()
            .data(ritcoin_state.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource(BROADCAST_RESOURCE).route(web::post().to(handle_submit_tx)))
            .service(web::resource(PENDINGS_RESOURCE).route(web::post().to(handle_pendings)))
            .service(web::resource(CHAIN_RESOURCE).route(web::post().to(handle_chain)))
            .service(web::resource(NODES_RESOURCE).route(web::post().to(handle_nodes)))
            .service(
                web::resource(CHAIN_LENGTH_RESOURCE).route(web::post().to(handle_chain_length)),
            )
    })
    .bind((DEFAULT_IP, port))?
    .run()
}
