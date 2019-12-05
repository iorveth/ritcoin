use crate::block::Block;
use crate::errors::*;
use crate::pending_pool;
use crate::serializer;
use crate::server::{CHAIN_RESOURCE, DEFAULT_ADDRESS};
use crate::transaction::*;
use crate::utxo_set::*;
use crate::wallet;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::net::SocketAddrV4;
use std::time::Instant;

const DEFAULT_DIFFICULTY: usize = 3;
const MINER_KEY_PATH: &str = "data/miner_key.txt";
const BLOCK_TRANSACTIONS_COUNT: usize = 3;
const DEFAULT_COINBASE_AMOUNT: u64 = 100;

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockChain {
    blocks: Vec<Block>,
    nodes: Vec<SocketAddrV4>,
    utxo: UtxoSet,
}

impl BlockChain {
    pub fn new() -> Self {
        if let Ok(existing_blockchain) = Self::exist(DEFAULT_ADDRESS) {
            existing_blockchain
        } else {
            Self {
                blocks: vec![],
                nodes: vec![],
                utxo: UtxoSet::new(),
            }
        }
    }

    pub fn get_utxos_ref(&self) -> &UtxoSet {
        &self.utxo
    }

    pub fn get_utxos_mut_ref(&mut self) -> &mut UtxoSet {
        &mut self.utxo
    }

    pub fn exist(address: &str) -> Result<Self, RitCoinErrror<'static>> {
        let client = Client::new();
        let chain_url = address.to_owned() + CHAIN_RESOURCE;
        let mut res = client.post(&chain_url).send()?;
        if res.status() == StatusCode::OK {
            Ok(res.json()?)
        } else {
            Err(RitCoinErrror::from(res.text()?))
        }
    }

    pub fn mine(&mut self) -> Result<(), RitCoinErrror<'static>> {
        if self.blocks.is_empty() {
            let genesis_block = Self::genesis_block(self.len() as u32)?;
            Ok(self.start_mine(genesis_block))
        } else {
            let private_key = wallet::wif_to_private_key_from_file(MINER_KEY_PATH)?;
            let public_key = wallet::private_key_to_public_key(&private_key)?;
            let pub_address = wallet::get_address(&public_key)?;
            let pk_hash = wallet::address_to_pkhash(&pub_address)?;
            let mut pending_transactions =
                pending_pool::get_last_transactions(Some(BLOCK_TRANSACTIONS_COUNT))?;
            let coinbase_transaction = Self::get_coinbase_transaction(
                &pk_hash,
                self.len() as u32,
                DEFAULT_COINBASE_AMOUNT,
            )?;
            pending_transactions.insert(0, coinbase_transaction);
            let mut deserialised_transactions = vec![];
            for pending_transaction in &pending_transactions {
                let deserialised_transaction = serializer::deserialize(pending_transaction)?;
                deserialised_transactions.push(deserialised_transaction);
            }
            let block = Block::new(
                self.blocks[self.blocks.len() - 1].hash(),
                pending_transactions,
            );
            block.validate_transactions(&self.utxo)?;
            self.start_mine(block);
            pending_pool::delete_last_n_transactions(BLOCK_TRANSACTIONS_COUNT)?;
            self.utxo.recalculate_utxos(&deserialised_transactions);
            Ok(())
        }
    }

    pub fn start_mine(&mut self, mut block: Block) {
        let mut start = Instant::now();
        while !block.hash().starts_with(&[0; DEFAULT_DIFFICULTY]) {
            block.increment_nonce();
            if start.elapsed().as_secs() == 2 {
                block.update_timestamp();
                start = Instant::now();
            }
        }
        self.blocks.push(block)
    }

    pub fn resolve_conflicts(&mut self) -> Result<(), RitCoinErrror<'static>> {
        for node_address in &self.nodes {
            if let Ok(node) = Self::exist(&node_address.to_string()) {
                if node.len() > self.len() && node.verify_chain().is_ok() {
                    self.blocks = node.blocks
                }
            }
        }
        Ok(())
    }

    pub fn verify_chain(&self) -> Result<(), RitCoinErrror<'static>> {
        self.blocks[0].validate_transactions(&self.utxo)?;
        for i in 0..self.blocks.len() - 1 {
            self.blocks[i+1].validate_transactions(&self.utxo)?;
            if !(self.blocks[i].hash() == self.blocks[i + 1].get_previous_hash()) {
                return Err(RitCoinErrror::from(
                    "previous block hash in next block do not match current block hash",
                ));
            }
        }
        Ok(())
    }

    pub fn get_coinbase_transaction(
        pk_hash: &[u8],
        block_height: u32,
        coinbase_amount: u64,
    ) -> Result<Vec<u8>, RitCoinErrror<'static>> {
        let coinbase_transaction = CoinBaseTransaction::new(pk_hash, block_height, coinbase_amount);
        serializer::serialize(&coinbase_transaction)
    }

    pub fn genesis_block(block_height: u32) -> Result<Block, RitCoinErrror<'static>> {
        let private_key = wallet::wif_to_private_key_from_file(MINER_KEY_PATH)?;
        let public_key = wallet::private_key_to_public_key(&private_key)?;
        let pub_address = wallet::get_address(&public_key)?;
        let pk_hash = wallet::address_to_pkhash(&pub_address)?;
        let coinbase_transaction_serialized =
            Self::get_coinbase_transaction(&pk_hash, block_height, DEFAULT_COINBASE_AMOUNT)?;
        Ok(Block::new(
            vec![0; 32],
            vec![coinbase_transaction_serialized],
        ))
    }

    pub fn add_node(&mut self, node: &str) {
        if let Ok(node) = node.parse::<SocketAddrV4>() {
            self.nodes.push(node);
            println!("Node {:?} added succesfully", node);
        } else {
            eprintln!("Invalid URL format")
        }
    }

    pub fn get_nodes(&self) -> &[SocketAddrV4] {
        &self.nodes
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }

    pub fn get_balance(&self, address: &str) -> Result<u64, RitCoinErrror<'static>> {
        let pk_hash = wallet::address_to_pkhash(address)?;
        let utxos = self.utxo.by_pkhash(&pk_hash);
        Ok(UtxoSet::get_total_amount(&utxos))
    }
}
