use crate::block::Block;
use crate::errors::*;
use crate::serializer;
use crate::transaction::*;
use crate::wallet;
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::ops::Deref;

const DEFAULT_DIFFICULTY: usize = 2;
const MINER_KEY_PATH: &str = "data/miner_key.txt";

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockChain {
    blocks: Vec<Block>,
    nodes: Vec<SocketAddrV4>,
}

impl BlockChain {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            nodes: vec![],
        }
    }

    pub fn mine(&mut self, mut block: Block) {
        while !block.hash().starts_with(&[0; DEFAULT_DIFFICULTY]) {
            block.increment_nonce()
        }
        self.blocks.push(block)
    }

    // fn resolve_conflicts(&mut self) {
    //     for node in &self.nodes {
    //         if node.len() > self.blocks.len() {
    //             self.blocks = node.to_vec()
    //         }
    //     }
    // }

    pub fn is_valid(&self) -> Result<(), RitCoinErrror<'static>> {
        self.blocks[0].validate_transactions()?;
        for i in 0..self.blocks.len() - 1 {
            self.blocks[i + 1].validate_transactions()?;
            if !(self.blocks[i].hash() == self.blocks[i + 1].get_previous_hash()) {
                return Err(RitCoinErrror::from(
                    "previous block hash in next block do not match current block hash",
                ));
            }
        }
        Ok(())
    }

    pub fn genesis_block() -> Result<Block, RitCoinErrror<'static>> {
        let private_key = wallet::wif_to_private_key_from_file(MINER_KEY_PATH)?;
        let public_key = wallet::private_key_to_public_key(&private_key)?;
        let pub_address = wallet::get_address(&public_key)?;
        let coinbase_transaction = CoinBaseTransaction::new(pub_address);
        let coinbase_transaction_serialized =
            serializer::serialize(&coinbase_transaction, &public_key)?;
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

    pub fn get_len(&self) -> usize {
        self.blocks.len()
    }

    pub fn get_balance(&self, address: &str) -> Result<u32, RitCoinErrror<'static>> {
        let mut balance = 0;
        for block in &self.blocks {
            for tx in block.get_transactions() {
                let (transaction, _) = serializer::deserialize(tx)?;
                if transaction.get_recipient() == address {
                    balance += transaction.get_amount()
                }
                if transaction.get_sender() == address {
                    balance -= transaction.get_amount()
                }
            }
        }
        Ok(balance)
    }
}
