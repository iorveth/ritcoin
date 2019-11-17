use crate::errors::*;
use crate::transaction::Transaction;
use crate::{serializer, tx_validator};
pub use std::fs::{self, File};
pub use std::io::prelude::*;
use std::io::{BufRead, BufReader};

const PENDING_POOL_PATH: &str = "data/pending_pool.txt";
//const LAST_TRANSACTIONS_COUNT: usize = 3;

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

pub fn tx_str_to_vec(tx: &str) -> Vec<u8> {
    tx.replace('[', "")
        .replace(']', "")
        .split(" ,")
        .filter_map(|elem| elem.parse::<u8>().ok())
        .collect()
}

pub fn get_last_transactions(n: Option<usize>) -> Result<Vec<Vec<u8>>, RitCoinErrror<'static>> {
    let input = File::open(PENDING_POOL_PATH)?;
    let buffered = BufReader::new(input);
    let mut transactions = vec![];
    if let Some(n) = n {
        for tx_str in buffered.lines().take(n) {
            let tx = tx_str_to_vec(&tx_str?);
            transactions.push(tx)
        }
    } else {
        for tx_str in buffered.lines() {
            let tx = tx_str_to_vec(&tx_str?);
            transactions.push(tx)
        }
    }
    Ok(transactions)
}

pub fn get_last_transactions_deserialized(
    n: Option<usize>,
) -> Result<Vec<Transaction>, RitCoinErrror<'static>> {
    let serialized_transactions = get_last_transactions(n)?;
    let mut deserialized_transactions = vec![];
    for tx in serialized_transactions {
        let (deserialized_tx, _) = serializer::deserialize(&tx)?;
        deserialized_transactions.push(deserialized_tx);
    }
    Ok(deserialized_transactions)
}
