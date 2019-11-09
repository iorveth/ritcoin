use crate::errors::*;
use crate::serializer;
use crate::transaction::*;
use crate::tx_validator;
use crate::wallet;
pub use std::fs::{self, File};
pub use std::io::prelude::*;
use std::io::stdin;

const ADDRESS_PATH: &str = "data/address.txt";
const PRIVATE_KEY_PATH: &str = "data/private_key.txt";

pub fn write_pub_address_to_file(pub_address: &str) -> Result<(), RitCoinErrror<'static>> {
    let mut file = File::create(ADDRESS_PATH)?;
    write!(file, "{}", pub_address)?;
    Ok(())
}

pub fn new() -> Result<(), RitCoinErrror<'static>> {
    let (private_key, public_key) = wallet::generate_ecdsa_key_pair();
    let pub_address = wallet::get_address(&public_key.serialize_uncompressed())?;
    println!("{:?}", private_key);
    write_pub_address_to_file(&pub_address)
}

pub fn import(path: &str) -> Result<(), RitCoinErrror<'static>> {
    let private_key = wallet::wif_to_private_key_from_file(path)?;
    let public_key = wallet::private_key_to_public_key(&private_key)?;
    let pub_address = wallet::get_address(&public_key)?;
    println!("{:?}", private_key);
    write_pub_address_to_file(&pub_address)
}

pub fn send(recipient_address: &str, amount: u32) -> Result<(), RitCoinErrror<'static>> {
    let sender_adress = fs::read_to_string(ADDRESS_PATH)?;
    let private_key_wif = fs::read_to_string(PRIVATE_KEY_PATH)?;
    let private_key = wallet::wif_to_private_key(private_key_wif)?;
    let mut transaction = Transaction::new(sender_adress, recipient_address.to_owned(), amount);
    let (signature, public_key) = wallet::sign(&transaction.hash(), &private_key)?;
    transaction.append_signature(signature);
    tx_validator::validate(&transaction, &public_key)?;
    let serialized = serializer::serialize(&transaction, &public_key)?;
    Ok(())
}

fn read_cli(command: &str) -> Result<(), RitCoinErrror<'static>> {
    match command {
        "new" => new(),
        command if command.starts_with("import") => {
            let path = command.split_ascii_whitespace().collect::<Vec<&str>>()[1];
            import(path)
        }
        command if command.starts_with("send") => {
            let command = command.replace(',', "");
            let send_parameters = command.split_ascii_whitespace().collect::<Vec<&str>>();
            let recipient_address = send_parameters[1];
            let amount = send_parameters[2].parse::<u32>()?;
            send(recipient_address, amount)
        }
        _ => Ok(()),
    }
}

pub fn cli() -> Result<(), RitCoinErrror<'static>> {
    loop {
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => read_cli(buf.trim())?,
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
        }
    }
    Ok(())
}
