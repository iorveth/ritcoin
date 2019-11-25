use crate::cli::{ADDRESS_PATH, PRIVATE_KEY_PATH};
use crate::errors::*;
use crate::pending_pool;
use crate::serializer;
use crate::server::{BROADCAST_RESOURCE, DEFAULT_ADDRESS};
use crate::transaction::*;
use crate::tx_validator;
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
    println!("{:?}", private_key);
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

pub fn get_used_utxos(utxos: &mut Vec<Utxo>, amount: u64) -> Vec<&Utxo> {
    let exact_utxo = utxos
        .iter()
        .filter(|utxo| utxo.get_output().amount() == amount)
        .take(1)
        .collect::<Vec<_>>();
    if exact_utxo.is_empty() {
        let exact_utxos = utxos
            .iter()
            .filter(|utxo| utxo.get_output().amount() < amount)
            .collect::<Vec<_>>();
        if exact_utxos
            .iter()
            .map(|utxo| utxo.get_output().amount())
            .sum()
            == amount
        {
            return exact_utxos;
        } else {
            //TODO Implement more efficient algorithm for utxo select here
            utxos.sort_by(|a, b| a.get_output().amount().cmp(&b.get_output().amount()));
            for utxo in utxos {
                if utxo.get_output().amount() > amount {
                    return vec![utxo];
                }
            }
            let mut total_amount = 0;
            let mut i = 0;
            for utxo in utxos {
                if total_amount > amount {
                    return utxos.iter().take(i).collect();
                } else {
                    total_amount += utxo.get_output().amount();
                    i += 1;
                }
            }
            utxos.iter().take(i).collect()
        }
    } else {
        exact_utxo
    }
}

pub fn send(
    recipient_address: &str,
    amount: u64,
    prepared_transactions: &mut Vec<Vec<u8>>,
    ritcoin_state: Arc<RitCoinState>,
) -> Result<(), RitCoinErrror<'static>> {
    let sender_adress = fs::read_to_string(ADDRESS_PATH)?;
    let private_key_wif = fs::read_to_string(PRIVATE_KEY_PATH)?;
    let private_key = wallet::wif_to_private_key(private_key_wif)?;
    let pkhash = wallet::address_to_pkhash(&sender_adress)?;
    if let Ok(blockchain_state) = ritcoin_state.blockchain.lock() {
        let utxos = blockchain_state.get_utxos_ref().by_pkhash(&pkhash);
        let used_utxos = get_used_utxos(&mut utxos, amount);
        let mut transaction = Transaction::new(sender_adress, recipient_address.to_owned(), amount);
        let (signature, public_key) = wallet::sign(&transaction.hash(), &private_key)?;
        transaction.append_signature(signature);
        tx_validator::validate(&transaction, &public_key)?;
        let serialized = serializer::serialize(&transaction, &public_key)?;
        println!("{:?}", serialized);
        prepared_transactions.push(serialized);
        Ok(())
    } else {
        Err(RitCoinErrror::from(
            "Error, when accessing blockchain state occured",
        ))
    }
}

pub fn broadcast(
    serialized_tx: &str,
    prepared_transactions: &mut Vec<Vec<u8>>,
) -> Result<(), RitCoinErrror<'static>> {
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
) -> Result<u32, RitCoinErrror<'static>> {
    if let Ok(blockchain_state) = ritcoin_state.blockchain.lock() {
        let balance = blockchain_state.get_balance(address)?;
        println!("{:?}", balance);
        Ok(balance)
    } else {
        Err(RitCoinErrror::from(
            "Error, when accessing blockchain state occured",
        ))
    }
}
