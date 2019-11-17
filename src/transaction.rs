use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u32,
    signature: Vec<u8>,
}

pub trait CoinBaseTransaction {
    fn new(recipient: String) -> Self;
}

impl CoinBaseTransaction for Transaction {
    fn new(recipient: String) -> Self {
        Self {
            sender: "0".repeat(34),
            recipient,
            amount: 50,
            signature: Vec::default(),
        }
    }
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u32) -> Self {
        Self {
            sender,
            recipient,
            amount,
            signature: Vec::default(),
        }
    }

    pub fn get_sender(&self) -> &str {
        &self.sender
    }

    pub fn get_recipient(&self) -> &str {
        &self.recipient
    }

    pub fn get_signature(&self) -> &[u8] {
        &self.signature
    }

    pub fn get_amount(&self) -> u32 {
        self.amount
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(&self.sender);
        hasher.input(&self.recipient);
        hasher.input(self.amount.to_string());
        hasher.result().to_vec()
    }

    pub fn append_signature(&mut self, signature: Vec<u8>) {
        self.signature = signature
    }
}
