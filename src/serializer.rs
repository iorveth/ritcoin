use crate::errors::*;
use crate::transaction::*;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SerializationFormat<'a> {
    amount: &'a str,
    sender: &'a str,
    recipient: &'a str,
    public_key: &'a str,
    signature: &'a [u8],
}

pub fn serialize(
    transaction: &Transaction,
    public_key: &[u8],
) -> Result<Vec<u8>, RitCoinErrror<'static>> {
    let serialization_format = SerializationFormat {
        amount: &format!("{:x}", transaction.get_amount()),
        sender: transaction.get_sender(),
        recipient: transaction.get_recipient(),
        public_key: &hex::encode(public_key),
        signature: transaction.get_signature(),
    };
    Ok(bincode::serialize(&serialization_format)?)
}

pub fn deserialize(serialized: &[u8]) -> Result<(Transaction, Vec<u8>), RitCoinErrror<'static>> {
    let deserialized: SerializationFormat = bincode::deserialize(serialized)?;
    let mut transaction = Transaction::new(
        deserialized.sender.to_owned(),
        deserialized.recipient.to_owned(),
        u32::from_str_radix(deserialized.amount, 16)?,
    );
    transaction.append_signature(deserialized.signature.to_owned());
    Ok((transaction, hex::decode(deserialized.public_key)?))
}
