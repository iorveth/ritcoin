mod errors;
mod pending_pool;
mod serializer;
mod transaction;
mod tx_validator;
mod wallet;
mod wallet_cli;
use errors::*;
use wallet_cli::*;

fn main() {
    if let Err(e) = wallet_cli::cli() {
        println!("{:?}", e)
    };
}
