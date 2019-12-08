use crate::cli::{ADDRESS_PATH, PRIVATE_KEY_PATH};
use crate::errors::*;
use crate::pending_pool;
use crate::serializer;
use crate::server::{BROADCAST_RESOURCE, DEFAULT_ADDRESS};
use crate::transaction::*;
use crate::utxo_set::*;
use crate::wallet;
use crate::*;
use reqwest::{Client, StatusCode};
use std::collections::HashMap;
pub use std::fs::{self, File};
pub use std::io::prelude::*;

pub fn write_pub_address_to_file(
    pub_address: &str,
    path_to_pub_address: &str,
) -> Result<(), RitCoinErrror<'static>> {
    let mut file = File::create(path_to_pub_address)?;
    write!(file, "{}", pub_address)?;
    Ok(())
}

pub fn new(path_to_pub_address: &str) -> Result<(), RitCoinErrror<'static>> {
    let (private_key, public_key) = wallet::generate_ecdsa_key_pair();
    let pub_address = wallet::get_address(&public_key.serialize_uncompressed())?;
    let private_key_wif = wallet::private_key_to_wif(&format!("{}", private_key))?;
    println!("{}", private_key_wif);
    write_pub_address_to_file(&pub_address, path_to_pub_address)
}

pub fn import(
    path_to_private_key: &str,
    path_to_pub_address: &str,
) -> Result<(), RitCoinErrror<'static>> {
    let private_key = wallet::wif_to_private_key_from_file(path_to_private_key)?;
    let public_key = wallet::private_key_to_public_key(&private_key)?;
    let pub_address = wallet::get_address(&public_key)?;
    println!("{:?}", private_key);
    write_pub_address_to_file(&pub_address, path_to_pub_address)
}

pub fn send(
    receiver_address: &str,
    amount: u64,
    prepared_transactions: &mut Vec<Vec<u8>>,
    ritcoin_state: Arc<RitCoinState>,
) -> Result<(), RitCoinErrror<'static>> {
    let sender_adress = fs::read_to_string(ADDRESS_PATH)?;
    let private_key_wif = fs::read_to_string(PRIVATE_KEY_PATH)?;
    let private_key = wallet::wif_to_private_key(&private_key_wif)?;
    let sender_pkhash = wallet::address_to_pkhash(&sender_adress)?;
    let receiver_pkhash = wallet::address_to_pkhash(&receiver_address)?;
    let mut prepared_transactions_deserialized = Vec::with_capacity(prepared_transactions.len());
    for tx in prepared_transactions.iter() {
        if !tx.is_empty() {
            let deserialized_tx = serializer::deserialize(tx)?;
            prepared_transactions_deserialized.push(deserialized_tx)
        }
    }
    if let Ok(blockchain_state) = ritcoin_state.blockchain.lock() {
        if let Some(used_utxos) = blockchain_state.get_utxos_ref().get_used_utxos(
            &sender_pkhash,
            amount,
            &prepared_transactions_deserialized,
        ) {
            let inputs = Input::create_inputs(&used_utxos);
            let utxo_total = UtxoSet::get_total_amount(&used_utxos);
            let outputs =
                Output::create_single(amount, utxo_total, &sender_pkhash, &receiver_pkhash);
            let mut transaction = Transaction::new(inputs, outputs);
            transaction.sign(&private_key)?;
            transaction.validate(&used_utxos)?;
            let serialized = serializer::serialize(&transaction)?;
            println!("{:?}", serialized);
            prepared_transactions.push(serialized);
            Ok(())
        } else {
            Err(RitCoinErrror::from(
                "Not enought utxo`s to create transaction!",
            ))
        }
    } else {
        Err(RitCoinErrror::from(
            "Error, when accessing blockchain state occured",
        ))
    }
}

pub fn unlock_all(prepared_transactions: &mut Vec<Vec<u8>>) {
    *prepared_transactions = vec![];
}

pub fn unlock(serialized_tx: &str, prepared_transactions: &mut Vec<Vec<u8>>) {
    prepared_transactions
        .iter()
        .position(|tx| *tx == pending_pool::tx_str_to_vec(serialized_tx))
        .map(|i| prepared_transactions.remove(i));
}

pub fn broadcast(
    serialized_tx: &str,
    prepared_transactions: &mut Vec<Vec<u8>>,
    testnet_option: bool,
) -> Result<(), RitCoinErrror<'static>> {
    println!("{:?}", serialized_tx);
    let tx = prepared_transactions
        .iter()
        .position(|tx| *tx == pending_pool::tx_str_to_vec(serialized_tx))
        .map(|i| prepared_transactions.remove(i));
    if let Some(tx) = &tx {
        let client = Client::new();
        let url = DEFAULT_ADDRESS.to_owned() + BROADCAST_RESOURCE;
        let mut map = HashMap::new();
        map.insert("tx", tx);
        let mut res = client.post(&url).json(&map).send()?;
        if res.status() == StatusCode::OK {
            println!("{}", res.text()?);
            Ok(())
        } else {
            Err(RitCoinErrror::from(res.text()?))
        }
    } else {
        Err(RitCoinErrror::from("Transaction not found"))
    }
}

pub fn balance(
    address: &str,
    ritcoin_state: Arc<RitCoinState>,
) -> Result<u64, RitCoinErrror<'static>> {
    if let Ok(blockchain_state) = ritcoin_state.blockchain.lock() {
        let balance = blockchain_state.get_balance(address)?;
        println!("{}", balance);
        Ok(balance)
    } else {
        Err(RitCoinErrror::from(
            "Error, when accessing blockchain state occured",
        ))
    }
}
