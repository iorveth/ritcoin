mod block;
mod blockchain;
mod cli;
mod errors;
mod handlers;
mod hash;
mod merkle;
mod miner_cli;
mod opcodes;
mod pending_pool;
mod script;
mod serializer;
mod server;
mod transaction;
mod utxo_set;
mod wallet;
mod wallet_cli;
use blockchain::BlockChain;
use cli::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct RitCoinState {
    blockchain: Mutex<BlockChain>,
}

impl RitCoinState {
    fn new() -> Self {
        Self {
            blockchain: Mutex::new(BlockChain::new()),
        }
    }
}

fn main() -> std::io::Result<()> {
    let ritcoin_state = Arc::new(RitCoinState::new());
    let ritcoin_state_cloned = ritcoin_state.clone();
    thread::spawn(move || {
        if let Err(e) = cli(ritcoin_state_cloned) {
            println!("{:?}", e)
        }
    });
    server::run(ritcoin_state)?;
    Ok(())
}
