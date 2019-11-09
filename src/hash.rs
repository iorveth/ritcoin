use ripemd160::Ripemd160;
use sha2::{Digest, Sha256};

pub fn sha256(value: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.input(value);
    hasher.result().to_vec()
}

pub fn ripemd160(value: &[u8]) -> Vec<u8> {
    let mut hasher = Ripemd160::new();
    hasher.input(value);
    hasher.result().to_vec()
}
