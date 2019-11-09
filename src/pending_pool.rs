use crate::errors::*;
use crate::{serializer, tx_validator};
pub use std::fs::{self, File};
pub use std::io::prelude::*;
use std::io::{BufRead, BufReader};

const PENDING_POOL_PATH: &str = "data/pending_pool.txt";
const LAST_TRANSACTIONS_COUNT: usize = 3;

fn save_to_mempool(serialized_transaction: &[u8]) -> Result<(), RitCoinErrror<'static>> {
    let mut file = File::create(PENDING_POOL_PATH)?;
    writeln!(file, "{:?}", serialized_transaction)?;
    Ok(())
}

pub fn accept_serialized_transaction(
    serialized_transaction: &[u8],
) -> Result<(), RitCoinErrror<'static>> {
    let (transaction, public_key) = serializer::deserialize(serialized_transaction)?;
    tx_validator::validate(&transaction, &public_key)?;
    save_to_mempool(serialized_transaction)
}

pub fn get_last_transactions() -> Result<Vec<Vec<u8>>, RitCoinErrror<'static>> {
    let input = File::open(PENDING_POOL_PATH)?;
    let buffered = BufReader::new(input);
    let mut transactions = vec![];
    for tx_str in buffered.lines().take(LAST_TRANSACTIONS_COUNT) {
        let tx = tx_str?
            .replace('[', "")
            .replace(']', "")
            .split(" ,")
            .filter_map(|elem| elem.parse::<u8>().ok())
            .collect::<Vec<u8>>();
        transactions.push(tx)
    }
    Ok(transactions)
}
