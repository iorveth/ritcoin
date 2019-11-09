use crate::errors::*;
use crate::wallet_cli::*;
use ripemd160::Ripemd160;
use secp256k1::{rand::rngs::OsRng, Message, PublicKey, Secp256k1, SecretKey, Signature};
use sha2::{Digest, Sha256};

const NETWORK_ID: u8 = 0x00;
const VERSION_NUMBER: u8 = 0x80;

pub fn private_key_to_wif_from_file(path: &str) -> Result<String, RitCoinErrror<'static>> {
    let private_key = fs::read_to_string(path)?;
    private_key_to_wif(private_key)
}

pub fn wif_to_private_key_from_file(path: &str) -> Result<Vec<u8>, RitCoinErrror<'static>> {
    let private_key_wif = fs::read_to_string(path)?;
    Ok(wif_to_private_key(private_key_wif)?)
}

fn get_checksum(key: &[u8]) -> Result<Vec<u8>, RitCoinErrror<'static>> {
    let hasher = |value| {
        let mut hasher = Sha256::new();
        hasher.input(value);
        hasher.result()
    };
    let hash1 = hasher(key);
    let hash2 = hasher(&hash1[..]);
    Ok(hash2[..4].to_vec())
}

pub fn private_key_to_wif(key: String) -> Result<String, RitCoinErrror<'static>> {
    let mut key = hex::decode(key)?;
    key.insert(0, VERSION_NUMBER);
    let checksum = get_checksum(&key)?;
    key.extend_from_slice(&checksum);
    Ok(bs58::encode(key).into_string())
}

pub fn wif_to_private_key(key: String) -> Result<Vec<u8>, bs58::decode::Error> {
    let key = bs58::decode(key).into_vec()?;
    let (private_key, _) = key.split_at(key.len() - 4);
    Ok(private_key[1..].to_vec())
}

pub fn generate_ecdsa_key_pair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    secp.generate_keypair(&mut rng)
}

pub fn private_key_to_public_key(private_key: &[u8]) -> Result<Vec<u8>, secp256k1::Error> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key)?;
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    Ok(get_compressed_pub_key(&public_key))
}

pub fn sign(hash: &[u8], private_key: &[u8]) -> Result<(Vec<u8>, Vec<u8>), secp256k1::Error> {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key)?;
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let hash = Message::from_slice(hash)?;
    let sig = secp.sign(&hash, &secret_key).serialize_compact();
    Ok((sig.to_vec(), get_compressed_pub_key(&public_key)))
}

pub fn verify(hash: &[u8], signature: &[u8], public_key: &[u8]) -> Result<(), secp256k1::Error> {
    let secp = Secp256k1::new();
    let hash = Message::from_slice(hash)?;
    let signature = Signature::from_compact(signature)?;
    let public_key = PublicKey::from_slice(public_key)?;
    secp.verify(&hash, &signature, &public_key)
}

pub fn get_compressed_pub_key(public_key: &PublicKey) -> Vec<u8> {
    public_key.serialize_uncompressed().to_vec()
}

pub fn get_address(public_key: &[u8]) -> Result<String, RitCoinErrror<'static>> {
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
