use crate::block::*;
use crate::errors::*;
use crate::serializer;
use crate::transaction::*;
use crate::wallet;
use std::ops::Deref;

const DEFAULT_DIFFICULTY: usize = 2;
const MINER_KEY_PATH: &str = "data/miner_key.txt";

pub struct Node(Vec<Block>);

impl Deref for Node {
    type Target = Vec<Block>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct BlockChain {
    blocks: Vec<Block>,
    nodes: Vec<Node>,
}

impl BlockChain {
    pub fn mine(&mut self, mut block: Block) {
        while !block.hash().starts_with(&[0; DEFAULT_DIFFICULTY]) {
            block.increment_nonce()
        }
        self.blocks.push(block)
    }

    fn resolve_conflicts(&mut self) {
        for node in &self.nodes {
            if node.len() > self.blocks.len() {
                self.blocks = node.to_vec()
            }
        }
    }

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
}
