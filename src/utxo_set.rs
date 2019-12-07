use crate::transaction::{Input, Output, Transaction};
use serde::{Deserialize, Serialize};

const UTXO_SET_PATH: &str = "data/utxo_set.txt";

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Utxo {
    tx_id: Vec<u8>,
    index: u32,
    output: Output,
    locked: bool,
}

impl Utxo {
    pub fn new(tx_id: Vec<u8>, index: u32, output: Output) -> Self {
        Self {
            tx_id,
            index,
            output,
            locked: false,
        }
    }

    pub fn get_output(&self) -> &Output {
        &self.output
    }

    pub fn get_tx_id(&self) -> &[u8] {
        &self.tx_id
    }

    pub fn get_index(&self) -> u32 {
        self.index
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

    pub fn by_pkhash(&self, pkhash: &[u8]) -> Vec<&Utxo> {
        self.utxos
            .iter()
            .filter(|utxo| {
                utxo.get_output()
                    .get_script_pubkey()
                    .windows(pkhash.len())
                    .any(|hash| hash == pkhash)
            })
            .collect()
    }

    pub fn by_pkhash_mut(&mut self, pkhash: &[u8]) -> Vec<&mut Utxo> {
        self.utxos
            .iter_mut()
            .filter(|utxo| {
                utxo.get_output()
                    .get_script_pubkey()
                    .windows(pkhash.len())
                    .any(|hash| hash == pkhash)
            })
            .collect()
    }

    pub fn get_validation_data<'a>(
        utxos: &'a [&Utxo],
        tx_id: &[u8],
        index: u32,
    ) -> Option<(&'a [u8], u64)> {
        for utxo in utxos {
            if utxo.get_tx_id() == tx_id && index == index {
                return Some((
                    utxo.get_output().get_script_pubkey(),
                    utxo.get_output().get_amount(),
                ));
            }
        }
        None
    }

    pub fn remove(&mut self, input: &Input) {
        self.utxos
            .iter()
            .position(|utxo| {
                (utxo.get_tx_id(), utxo.get_index()) == input.get_previous_output().get()
            })
            .map(|i| self.utxos.remove(i));
    }

    pub fn remove_used(&mut self, tx_in: &[Input]) {
        tx_in.iter().for_each(|input| self.remove(input))
    }

    pub fn add_unspent(&mut self, tx_out: &[Output], tx_id: Vec<u8>) {
        tx_out.iter().enumerate().for_each(|(index, output)| {
            let utxo = Utxo::new(tx_id.clone(), index as u32, output.clone());
            self.utxos.push(utxo);
        })
    }

    pub fn recalculate_utxos(&mut self, transactions: &[Transaction]) {
        for transaction in transactions {
            self.remove_used(transaction.get_tx_in());
            self.add_unspent(transaction.get_tx_out(), transaction.hash());
        }
    }

    pub fn get_used_utxos(&self, sender_pkhash: &[u8], amount: u64) -> Option<Vec<&Utxo>> {
        let mut utxos: Vec<_> = self
            .by_pkhash(sender_pkhash)
            .into_iter()
            .filter(|utxo| !utxo.locked)
            .collect();
        let exact_utxo = utxos
            .iter()
            .filter(|utxo| utxo.get_output().get_amount() == amount)
            .take(1)
            .map(|utxo| *utxo)
            .collect::<Vec<_>>();
        if exact_utxo.is_empty() {
            let exact_utxos = utxos
                .iter()
                .filter(|utxo| utxo.get_output().get_amount() < amount)
                .map(|utxo| *utxo)
                .collect::<Vec<_>>();
            if exact_utxos
                .iter()
                .map(|utxo| utxo.get_output().get_amount())
                .sum::<u64>()
                == amount
            {
                return Some(exact_utxos);
            } else {
                //TODO Implement more efficient algorithm for utxo selection here
                utxos.sort_by(|a, b| {
                    a.get_output()
                        .get_amount()
                        .cmp(&b.get_output().get_amount())
                });
                for utxo in &utxos {
                    if utxo.get_output().get_amount() > amount {
                        return Some(vec![utxo]);
                    }
                }
                let mut total_amount = 0;
                let mut i = 0;
                for utxo in &utxos {
                    if total_amount > amount {
                        return Some(utxos.into_iter().take(i).collect());
                    } else {
                        total_amount += utxo.get_output().get_amount();
                        i += 1;
                    }
                }
                None
            }
        } else {
            Some(exact_utxo)
        }
    }

    pub fn get_total_amount(utxos: &[&Utxo]) -> u64 {
        utxos
            .iter()
            .fold(0, |acc, utxo| acc + utxo.get_output().get_amount())
    }

    pub fn lock_utxos(&mut self, sender_pkhash: &[u8], utxos: &[&Utxo]) {
        for utxo in self.by_pkhash_mut(sender_pkhash) {
            for selected_utxo in utxos {
                if *utxo == **selected_utxo {
                    utxo.locked = true;
                }
            }
        }
    }
}
