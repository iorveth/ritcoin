use crate::wallet_cli::*;
use std::io::BufReader;
use crate::RitCoinErrror;
use sha2::{Sha256, Digest};
use secp256k1::{rand::rngs::OsRng, SecretKey, Secp256k1, PublicKey, Message};
use ripemd160::Ripemd160;

const NETWORK_ID: u8 = 0x00;

pub fn private_key_to_WIF_from_file(path: &str) -> Result<String, RitCoinErrror> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    private_key_to_WIF(contents)
}

pub fn WIF_to_private_key_from_file(path: &str) -> Result<Vec<u8>, RitCoinErrror> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(WIF_to_private_key(contents)?)
}

fn get_checksum(key: &[u8]) -> Result<Vec<u8>, RitCoinErrror> {
    let hasher = |value| {
        let mut hasher = Sha256::new();
        hasher.input(value);
        hasher.result()
    };
    let hash1 = hasher(key);
    let hash2 = hasher(&hash1[..]);
    Ok(hash2[..4].to_vec())
}

fn private_key_to_WIF(key: String) -> Result<String, RitCoinErrror> {
    let mut key = hex::decode(key)?;
    key.insert(0, 0x80);
    let checksum = get_checksum(&key)?;
    key.extend_from_slice(&checksum);
    Ok(bs58::encode(key).into_string())
}

fn WIF_to_private_key(key: String) -> Result<Vec<u8>, bs58::decode::Error> {
    let key = bs58::decode(key).into_vec()?;
    let (private_key, _) = key.split_at(key.len() - 4);
    Ok(private_key[1..].to_vec())
}

pub fn generate_ecdsa_key_pair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    secp.generate_keypair(&mut rng)
}

pub fn private_key_to_public_key(private_key: &[u8]) -> Result<PublicKey, secp256k1::Error> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key)?;
    Ok(PublicKey::from_secret_key(&secp, &secret_key))
}

fn sign(tx_id: &[u8], private_key: &[u8]) -> Result<(String, String), secp256k1::Error> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key)?;
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let tx_id = Message::from_slice(tx_id)?;
    let sig = secp.sign(&tx_id, &secret_key)
        .serialize_compact();
    Ok((hex::encode(&sig[..]), hex::encode(get_compressed_pub_key(&public_key))))
}

pub fn get_compressed_pub_key(public_key: &PublicKey) -> Vec<u8> {
    public_key.serialize_uncompressed().to_vec()
}

pub fn get_address(public_key: &PublicKey) -> Result<String, RitCoinErrror> {
    let public_key = get_compressed_pub_key(public_key);
    let mut sha256_hasher = Sha256::new();
    sha256_hasher.input(public_key);
    let sha256_hash = sha256_hasher.result();
    let mut ripemd160_hasher = Ripemd160::new();
    ripemd160_hasher.input(&sha256_hash[..]);
    let mut encrypted_pub_key = ripemd160_hasher.result().to_vec();
    encrypted_pub_key.insert(0, NETWORK_ID);
    let checksum = get_checksum(&encrypted_pub_key)?;
    encrypted_pub_key.extend_from_slice(&checksum);
    Ok(bs58::encode(encrypted_pub_key).into_string())
}