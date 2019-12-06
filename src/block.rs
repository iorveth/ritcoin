use crate::errors::*;
use crate::merkle::*;
use crate::serializer;
use crate::utxo_set::UtxoSet;
use crate::wallet;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::time::SystemTime;

const BLOCK_VERSION: i32 = 1;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    version: i32,
    previous_block_header_hash: Vec<u8>,
    merkle_root: Vec<u8>,
    timestamp: u64,
    nonce: usize,
    transactions: Vec<Vec<u8>>,
}

impl Block {
    pub fn new(previous_block_header_hash: Vec<u8>, transactions: Vec<Vec<u8>>) -> Self {
        let merkle_root = get_merkle_root(&transactions);
        Self {
            version: BLOCK_VERSION,
            previous_block_header_hash,
            merkle_root,
            timestamp: Self::calculate_timestamp(),
            nonce: 0,
            transactions,
        }
    }

    pub fn validate_transactions(&self, utxo_set: &UtxoSet) -> Result<(), RitCoinErrror<'static>> {
        let pub_keys = self.pub_keys_from_txins()?;
        let pk_hashes: Vec<_> = pub_keys
            .iter()
            .map(|pub_key| wallet::pk_hash_from_public_key(pub_key))
            .collect();
        let utxos: Vec<_> = pk_hashes
            .iter()
            .map(|pk_hash| utxo_set.by_pkhash(pk_hash))
            .flatten()
            .collect();
        // Do not validate coinbase transaction
        for (i, transaction) in self.transactions.iter().enumerate() {
            if i != 0 {
                let transaction = serializer::deserialize(transaction)?;
                transaction.validate(&utxos)?;
            }
        }
        Ok(())
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(self.version.to_string());
        hasher.input(&self.previous_block_header_hash);
        hasher.input(&self.merkle_root);
        hasher.input(self.timestamp.to_string());
        hasher.input(self.nonce.to_string());
        hasher.result().to_vec()
    }

    pub fn get_previous_hash(&self) -> &[u8] {
        &self.previous_block_header_hash
    }

    pub fn get_transactions(&self) -> &[Vec<u8>] {
        &self.transactions
    }

    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
    }

    pub fn update_timestamp(&mut self) {
        self.timestamp = Self::calculate_timestamp();
    }

    pub fn pub_keys_from_txins(&self) -> Result<Vec<Vec<u8>>, RitCoinErrror<'static>> {
        let mut pub_keys_set = HashSet::new();
        for (i, transaction) in self.transactions.iter().enumerate() {
            if i != 0 {
                let transaction = serializer::deserialize(transaction)?;
                let pub_keys = transaction.get_pub_keys_from_inputs();
                pub_keys_set = pub_keys_set.union(&pub_keys).cloned().collect();
            }
        }
        Ok(pub_keys_set
            .into_iter()
            .map(|pub_key_set| pub_key_set.to_vec())
            .collect())
    }

    fn calculate_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs()
    }
}
