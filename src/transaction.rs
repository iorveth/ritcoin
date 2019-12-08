use crate::errors::*;
use crate::hash::*;
use crate::opcodes::*;
use crate::script;
use crate::utxo_set::{Utxo, UtxoSet};
use crate::wallet;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

const VERSION: i32 = 1;

const SIGHASH_ALL: u8 = 1;
const SIGHASH_NONE: u8 = 2;
const SIGHASH_SINGLE: u8 = 3;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutPoint {
    tx_id: Vec<u8>,
    index: u32,
}

impl OutPoint {
    pub fn new(tx_id: Vec<u8>, index: u32) -> Self {
        Self { tx_id, index }
    }

    pub fn get_tx_id(&self) -> &[u8] {
        &self.tx_id
    }

    pub fn get(&self) -> (&[u8], u32) {
        (&self.tx_id, self.index)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    previous_output: OutPoint,
    script_bytes: u16,
    sig_script: Vec<u8>,
    sequence: u32,
}

impl Input {
    pub fn create(utxo: &Utxo) -> Self {
        let previous_output = OutPoint::new(utxo.get_tx_id().to_vec(), utxo.get_index());
        let script_pubkey = utxo.get_output().get_script_pubkey();
        Self {
            previous_output,
            script_bytes: script_pubkey.len() as u16,
            sig_script: script_pubkey.to_vec(),
            sequence: std::u32::MAX,
        }
    }

    pub fn create_inputs(used_utxos: &[&Utxo]) -> Vec<Self> {
        let mut inputs = vec![];
        used_utxos
            .iter()
            .for_each(|used_utxo| inputs.push(Self::create(used_utxo)));
        inputs
    }

    pub fn get_sig_script(&self) -> &[u8] {
        &self.sig_script
    }

    pub fn get_public_key(&self) -> &[u8] {
        &self.sig_script[self.sig_script.len() - 65..]
    }

    pub fn hash(&self, hasher: &mut Sha256, sig_script: bool) {
        hasher.input(&self.previous_output.tx_id);
        hasher.input(self.previous_output.index.to_string());
        hasher.input(self.script_bytes.to_string());
        if sig_script {
            hasher.input(&self.sig_script);
        } else {
            hasher.input(0.to_string())
        }
        hasher.input(self.sequence.to_string());
    }

    pub fn get_previous_output(&self) -> &OutPoint {
        &self.previous_output
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Output {
    amount: u64,
    script_length: u16,
    script_pubkey: Vec<u8>,
}

impl Output {
    fn create(amount: u64, receiver_pkhash: &[u8]) -> Self {
        let mut script_pubkey = Vec::new();
        script_pubkey.push(OP_DUP);
        script_pubkey.push(OP_HASH160);
        script_pubkey.extend_from_slice(receiver_pkhash);
        script_pubkey.push(OP_EQUALVERIFY);
        script_pubkey.push(OP_CHECKSIG);
        Self {
            amount,
            script_length: script_pubkey.len() as u16,
            script_pubkey,
        }
    }

    pub fn create_single(
        amount: u64,
        utxo_total: u64,
        sender_pk_hash: &[u8],
        receiver_pkhash: &[u8],
    ) -> Vec<Self> {
        let mut outputs = vec![];
        let output = Self::create(amount, receiver_pkhash);
        outputs.push(output);
        if utxo_total - amount != 0 {
            let remainder = Self::create(utxo_total - amount, sender_pk_hash);
            outputs.push(remainder)
        }
        outputs
    }

    pub fn get_script_pubkey(&self) -> &[u8] {
        &self.script_pubkey
    }

    pub fn get_amount(&self) -> u64 {
        self.amount
    }

    pub fn hash(&self, hasher: &mut Sha256) {
        hasher.input(self.amount.to_string());
        hasher.input(self.script_length.to_string());
        hasher.input(&self.script_pubkey);
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    version: i32,
    tx_in_count: u16,
    tx_in: Vec<Input>,
    tx_out_count: u16,
    tx_out: Vec<Output>,
    lock_time: u32,
}

pub trait CoinBaseTransaction {
    fn new(receiver_pkhash: &[u8], block_height: u32, coinbase_amount: u64) -> Self;
}

impl CoinBaseTransaction for Transaction {
    fn new(receiver_pkhash: &[u8], block_height: u32, coinbase_amount: u64) -> Self {
        let previous_output = OutPoint::new(vec![0; 64], std::u32::MAX);
        let sig_script: Vec<_> = block_height
            .to_le_bytes()
            .into_iter()
            .filter(|i| **i != 0)
            .map(|i| i.to_owned())
            .collect();
        let tx_in = vec![Input {
            previous_output,
            script_bytes: sig_script.len() as u16,
            sig_script,
            sequence: std::u32::MAX,
        }];
        let tx_out = vec![Output::create(coinbase_amount, receiver_pkhash)];
        Self {
            version: VERSION,
            tx_in_count: 1,
            tx_in,
            tx_out_count: 1,
            tx_out,
            lock_time: 0,
        }
    }
}

impl Transaction {
    pub fn new(tx_in: Vec<Input>, tx_out: Vec<Output>) -> Self {
        Self {
            version: VERSION,
            tx_in_count: tx_in.len() as u16,
            tx_in,
            tx_out_count: tx_out.len() as u16,
            tx_out,
            lock_time: 0,
        }
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(&self.version.to_string());
        hasher.input(&self.tx_in_count.to_string());
        for tx_in in &self.tx_in {
            tx_in.hash(&mut hasher, true)
        }
        for tx_out in &self.tx_out {
            tx_out.hash(&mut hasher)
        }
        let sha_256_hash = hasher.result().to_vec();
        sha256(&sha_256_hash)
    }

    pub fn hash_one(&self, input: &Input) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.input(&self.version.to_string());
        hasher.input(&self.tx_in_count.to_string());
        for tx_in in &self.tx_in {
            if *input != *tx_in {
                tx_in.hash(&mut hasher, false)
            } else {
                tx_in.hash(&mut hasher, true)
            }
        }
        for tx_out in &self.tx_out {
            tx_out.hash(&mut hasher)
        }
        hasher.input(SIGHASH_ALL.to_string());
        let sha_256_hash = hasher.result().to_vec();
        sha256(&sha_256_hash)
    }

    pub fn hash_all(&self) -> Vec<Vec<u8>> {
        self.tx_in
            .iter()
            .map(|input| self.hash_one(input))
            .collect()
    }

    pub fn calculate_sig_script(signature: &[u8], pub_key: &[u8]) -> Vec<u8> {
        let mut sig_script = vec![];
        sig_script.push((signature.len() + 1) as u8);
        sig_script.extend_from_slice(signature);
        sig_script.push(1);
        sig_script.push(pub_key.len() as u8);
        sig_script.extend_from_slice(pub_key);
        sig_script
    }

    pub fn sign(&mut self, private_key: &[u8]) -> Result<(), RitCoinErrror<'static>> {
        let hashes = self.hash_all();
        for (i, hash) in hashes.iter().enumerate() {
            let (signature, pub_key) = wallet::sign(hash, private_key)?;
            self.tx_in[i].sig_script = Self::calculate_sig_script(&signature, &pub_key);
            self.tx_in[i].script_bytes = self.tx_in[i].sig_script.len() as u16;
        }
        Ok(())
    }

    pub fn get_original_hashes(&self, utxos: &[&Utxo]) -> Vec<Vec<u8>> {
        let mut transaction = self.clone();
        for input in &mut transaction.tx_in {
            if let Some((script_pubkey, _)) = UtxoSet::get_validation_data(
                utxos,
                &input.previous_output.tx_id,
                input.previous_output.index,
            ) {
                input.script_bytes = script_pubkey.len() as u16;
                input.sig_script = script_pubkey.to_vec();
            }
        }
        transaction.hash_all()
    }

    pub fn validate(&self, utxos: &[&Utxo]) -> Result<(), RitCoinErrror<'static>> {
        let mut inputs_sum = 0;
        let hashes = self.get_original_hashes(utxos);
        for (i, input) in self.tx_in.iter().enumerate() {
            if let Some((script_pubkey, amount)) = UtxoSet::get_validation_data(
                utxos,
                &input.previous_output.tx_id,
                input.previous_output.index,
            ) {
                inputs_sum += amount;
                script::execute(input.get_sig_script(), script_pubkey, &hashes[i])?;
            }
        }
        let outputs_sum = self
            .tx_out
            .iter()
            .fold(0_u64, |acc, output| acc + output.amount);
        if inputs_sum >= outputs_sum {
            Ok(())
        } else {
            Err(RitCoinErrror::from(
                "Sum of all inputs is less than sum of outputs",
            ))
        }
    }

    pub fn verify(&self, utxos: &[&Utxo]) -> Result<(), RitCoinErrror<'static>> {
        let hashes = self.get_original_hashes(utxos);
        for (i, input) in self.tx_in.iter().enumerate() {
            if let Some((script_pubkey, _)) = UtxoSet::get_validation_data(
                utxos,
                &input.previous_output.tx_id,
                input.previous_output.index,
            ) {
                script::execute(input.get_sig_script(), script_pubkey, &hashes[i])?;
            }
        }
        Ok(())
    }

    pub fn get_pub_keys_from_inputs(&self) -> HashSet<Vec<u8>> {
        let mut pub_key_set = HashSet::new();
        for input in &self.tx_in {
            pub_key_set.insert(input.get_public_key().to_vec());
        }
        pub_key_set
    }

    pub fn get_tx_in(&self) -> &[Input] {
        &self.tx_in
    }

    pub fn get_tx_out(&self) -> &[Output] {
        &self.tx_out
    }
}
