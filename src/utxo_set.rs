use crate::transaction::Output;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const UTXO_SET_PATH: &str = "data/utxo_set.txt";

#[derive(Serialize, Deserialize, Clone)]
pub struct Utxo {
    tx_id: Vec<u8>,
    index: usize,
    output: Output,
}
impl Utxo {
    pub fn new(tx_id: Vec<u8>, index: usize, output: Output) -> Self {
        Self {
            tx_id,
            index,
            output,
        }
    }

    pub fn get_output(&self) -> &Output {
        &self.output
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct UtxoSet {
    utxos: Vec<Utxo>,
}

impl UtxoSet {
    pub fn new() -> Self {
        Self { utxos: vec![] }
    }

    pub fn add_utxo(&mut self, tx_id: Vec<u8>, index: usize, output: Output) {
        let utxo = Utxo::new(tx_id, index, output);
        self.utxos.push(utxo);
    }

    pub fn by_pkhash(&self, pkhash: &str) -> Vec<Utxo> {
        self.utxos
            .into_iter()
            .filter(|utxo| utxo.output.get_script_pubkey().contains(pkhash))
            .collect()
    }
}
