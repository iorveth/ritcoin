use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const VERSION: u8 = 1;
#[derive(Debug, Serialize, Deserialize)]
pub struct OutPoint {
    hash: Vec<u8>,
    index: u32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    previous_output: OutPoint,
    script_bytes: u16,
    sig_script: String,
    sequence: u32,
}

pub struct CoinBaseInput {
    hash: Vec<u8>,
    index: u32,
    script_bytes: u16,
    height: String,
    sig_script: Option<String>,
    sequence: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Output {
    amount: u64,
    script_length: u16,
    script_pubkey: String,
}

impl Output {
    pub fn get_script_pubkey(&self) -> &str {
        &self.script_pubkey
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    version: i32,
    tx_in_count: u16,
    tx_in: Vec<Input>,
    tx_out_count: u16,
    tx_out: Vec<Output>,
    lock_time: u32,
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
    pub fn new(inputs: Vec<Input>, outputs: Vec<Output>, amount: u32) -> Self {
        Self {
            version: VERSION,
            input_counter: iputs.len(),
            inputs,
            output_counter: outputs.len(),
            outputs,
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
