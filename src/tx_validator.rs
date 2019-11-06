use crate::transaction::*;
use crate::{errors::*, wallet};

pub fn validate_address(address: &str) -> bool {
    address.len() == 34 && (address == "0".repeat(34) || bs58::decode(address).into_vec().is_ok())
}

pub fn validate_addresses(
    transaction: &Transaction,
    public_key: &[u8],
) -> Result<bool, RitCoinErrror> {
    if validate_address(transaction.get_sender()) && validate_address(transaction.get_recipient()) {
        Ok(transaction.get_sender() == wallet::get_address(public_key)?)
    } else {
        Ok(false)
    }
}

pub fn validate_signature(
    transaction: &Transaction,
    public_key: &[u8],
) -> Result<(), secp256k1::Error> {
    wallet::verify(&transaction.hash(), transaction.get_signature(), public_key)
}

pub fn validate(transaction: &Transaction, public_key: &[u8]) -> Result<bool, RitCoinErrror> {
    match (
        validate_addresses(transaction, public_key)?,
        validate_signature(transaction, public_key)?,
    ) {
        (true, _) => Ok(true),
        (false, _) => Ok(false),
    }
}
