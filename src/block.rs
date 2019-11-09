//use std::time::SystemTime;
use crate::errors::*;
use crate::merkle::*;
use crate::{serializer, tx_validator};
use sha2::{Digest, Sha256};

pub struct Block {
    timestamp: usize,
    nonce: usize,
    previous_hash: String,
    transactions: Vec<Vec<u8>>,
    merkle_root: Vec<u8>,
    hash: Vec<u8>,
}

impl Block {
    pub fn new(
        timestamp: usize,
        nonce: usize,
        previous_hash: String,
        transactions: Vec<Vec<u8>>,
    ) -> Self {
        let merkle_root = get_merkle_root(&transactions);
        let hash = Self::calculate_hash(
            timestamp,
            nonce,
            &previous_hash,
            &transactions,
            &merkle_root,
        );
        Self {
            timestamp,
            nonce,
            previous_hash,
            transactions,
            merkle_root,
            hash,
        }
    }

    pub fn validate_transactions(&self) -> Result<(), RitCoinErrror> {
        for transaction in &self.transactions {
            let (transaction, public_key) = serializer::deserialize(transaction)?;
            tx_validator::validate(&transaction, &public_key)?;
        }
        Ok(())
    }

    fn calculate_hash(
        timestamp: usize,
        nonce: usize,
        previous_hash: &str,
        transactions: &[Vec<u8>],
        merkle_root: &[u8],
    ) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(timestamp.to_string());
        hasher.input(nonce.to_string());
        hasher.input(previous_hash);
        transactions
            .iter()
            .for_each(|transaction| hasher.input(transaction));
        hasher.input(merkle_root);
        hasher.result().to_vec()
    }
}
