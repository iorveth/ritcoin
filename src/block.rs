use crate::errors::*;
use crate::merkle::*;
use crate::{serializer, tx_validator};
use sha2::{Digest, Sha256};
use std::time::SystemTime;

#[derive(Clone)]
pub struct Block {
    timestamp: u64,
    nonce: usize,
    previous_hash: Vec<u8>,
    transactions: Vec<Vec<u8>>,
    merkle_root: Vec<u8>,
}

impl Block {
    pub fn new(previous_hash: Vec<u8>, transactions: Vec<Vec<u8>>) -> Self {
        let merkle_root = get_merkle_root(&transactions);
        Self {
            timestamp: Self::calculate_timestamp(),
            nonce: 0,
            previous_hash,
            transactions,
            merkle_root,
        }
    }

    pub fn validate_transactions(&self) -> Result<(), RitCoinErrror<'static>> {
        for transaction in &self.transactions {
            let (transaction, public_key) = serializer::deserialize(transaction)?;
            tx_validator::validate(&transaction, &public_key)?;
        }
        Ok(())
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(self.timestamp.to_string());
        hasher.input(self.nonce.to_string());
        hasher.input(&self.previous_hash);
        self.transactions
            .iter()
            .for_each(|transaction| hasher.input(transaction));
        hasher.input(&self.merkle_root);
        hasher.result().to_vec()
    }

    pub fn get_previous_hash(&self) -> &[u8] {
        &self.previous_hash
    }

    pub fn get_transactions(&self) -> &[Vec<u8>] {
        &self.transactions
    }

    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
        self.update_timestamp()
    }

    fn calculate_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs()
    }

    fn update_timestamp(&mut self) {
        self.timestamp = Self::calculate_timestamp();
    }
}
