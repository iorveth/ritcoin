mod block;
mod blockchain;
mod cli;
mod errors;
mod hash;
mod merkle;
mod miner_cli;
mod pending_pool;
mod serializer;
mod transaction;
mod tx_validator;
mod wallet;
mod wallet_cli;
use cli::*;
use errors::*;
use hash::*;

fn main() {
    if let Err(e) = cli() {
        println!("{:?}", e)
    };
}
