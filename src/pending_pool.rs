use crate::errors::*;
use crate::transaction::Transaction;
use crate::{serializer, tx_validator};
pub use std::fs::{self, File, OpenOptions};
pub use std::io::prelude::*;
use std::io::{BufRead, BufReader};

const PENDING_POOL_PATH: &str = "data/pending_pool.txt";

fn save_to_mempool(serialized_transaction: &[u8]) -> Result<(), RitCoinErrror<'static>> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(PENDING_POOL_PATH)?;
    writeln!(file, "{:?}", serialized_transaction)?;
    Ok(())
}

pub fn delete_last_n_transactions(n: usize) -> Result<(), RitCoinErrror<'static>> {
    let data = fs::read_to_string(PENDING_POOL_PATH)?;
    File::create(PENDING_POOL_PATH)?;
    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .open(PENDING_POOL_PATH)?;
    for tx in data.lines() {
        writeln!(file, "{:?}", tx)?;
    }
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
