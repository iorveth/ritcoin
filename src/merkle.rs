use crate::hash::*;
use sha2::{Digest, Sha256};

fn merge(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.input(lhs);
    hasher.input(rhs);
    hasher.result().to_vec()
}

fn merkle_root(mut transaction_hashes: Vec<Vec<u8>>) -> Vec<u8> {
    match transaction_hashes.len() {
        0 => return vec![],
        1 => return transaction_hashes.remove(0),
        len if len % 2 == 1 => {
            transaction_hashes.push(transaction_hashes[transaction_hashes.len() - 1].clone())
        }
        _ => (),
    };
    let mut parent_hashes = vec![];
    for i in 0..transaction_hashes.len() - 1 {
        let parent = merge(&transaction_hashes[i], &transaction_hashes[i + 1]);
        parent_hashes.push(parent)
    }
    merkle_root(parent_hashes)
}

pub fn get_merkle_root(serialized_transactions: &[Vec<u8>]) -> Vec<u8> {
    let mut transaction_hashes = vec![];
    serialized_transactions
        .iter()
        .for_each(|serialized_transaction| transaction_hashes.push(sha256(serialized_transaction)));
    merkle_root(transaction_hashes)
}
