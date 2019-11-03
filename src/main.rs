mod wallet;
mod wallet_cli;
mod errors;
use wallet_cli::*;
use errors::RitCoinErrror;

fn main() {
    wallet_cli::cli();
}
