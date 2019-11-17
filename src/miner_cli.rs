use crate::errors::*;
use crate::wallet_cli;
use crate::*;

const MINER_ADDRESS_PATH: &str = "data/miner_address.txt";
const MINER_PRIVATE_KEY_PATH: &str = "data/miner_key.txt";

pub fn new() -> Result<(), RitCoinErrror<'static>> {
    wallet_cli::new(MINER_ADDRESS_PATH)
}

pub fn import(private_key_path: &str) -> Result<(), RitCoinErrror<'static>> {
    wallet_cli::import(private_key_path, MINER_ADDRESS_PATH)
}

pub fn add_node(
    node: &str,
    ritcoin_state: Arc<RitCoinState>,
) -> Result<(), RitCoinErrror<'static>> {
    if let Ok(mut blockchain_state) = ritcoin_state.blockchain.lock() {
        Ok(blockchain_state.add_node(node))
    } else {
        Err(RitCoinErrror::from("Error, when adding node occured"))
    }
}

pub fn mine(ritcoin_state: Arc<RitCoinState>) -> Result<(), RitCoinErrror<'static>> {
    if let Ok(mut blockchain_state) = ritcoin_state.blockchain.lock() {
        blockchain_state.mine()
    } else {
        Err(RitCoinErrror::from("Error, when adding node occured"))
    }
}
