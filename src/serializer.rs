use crate::errors::*;
use crate::transaction::*;

pub fn serialize(transaction: &Transaction) -> Result<Vec<u8>, RitCoinErrror<'static>> {
    Ok(bincode::serialize(transaction)?)
}

pub fn deserialize(serialized: &[u8]) -> Result<Transaction, RitCoinErrror<'static>> {
    let deserialized: Transaction = bincode::deserialize(serialized)?;
    Ok(deserialized)
}
