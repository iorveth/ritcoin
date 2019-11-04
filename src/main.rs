mod wallet;
mod wallet_cli;
mod errors;
use wallet_cli::*;
use errors::RitCoinErrror;

fn main() {
    if let Err(e) = wallet_cli::cli() {
        println!("{:?}", e)
    };
}
