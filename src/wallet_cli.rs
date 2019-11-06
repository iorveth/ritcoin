use crate::transaction::*;
use crate::tx_validator;
use crate::wallet;
use crate::RitCoinErrror;
pub use std::fs::{self, File};
pub use std::io::prelude::*;
use std::io::stdin;

const PATH_TO_ADDRESS: &str = "data/address.txt";
const PRIVATE_KEY: &str = "data/private_key.txt";

pub fn new() -> Result<(), RitCoinErrror> {
    let (private_key, public_key) = wallet::generate_ecdsa_key_pair();
    let pub_address = wallet::get_address(&public_key.serialize_uncompressed())?;
    println!("{:?}", private_key);
    let mut file = File::create(PATH_TO_ADDRESS)?;
    write!(file, "{}", pub_address);
    Ok(())
}

pub fn import(path: &str) -> Result<(), RitCoinErrror> {
    let private_key = wallet::WIF_to_private_key_from_file(path)?;
    let public_key = wallet::private_key_to_public_key(&private_key)?;
    let pub_address = wallet::get_address(&public_key)?;
    println!("{:?}", private_key);
    let mut file = File::create(PATH_TO_ADDRESS)?;
    write!(file, "{}", pub_address);
    Ok(())
}

pub fn send(recipient_address: &str, amount: usize) -> Result<(), RitCoinErrror> {
    let sender_adress = fs::read_to_string(PATH_TO_ADDRESS)?;
    let private_key_WIF = fs::read_to_string(PRIVATE_KEY)?;
    let private_key = wallet::WIF_to_private_key(private_key_WIF)?;
    let mut transaction = Transaction::new(sender_adress, recipient_address.to_owned(), amount);
    let (signature, public_key) = wallet::sign(&transaction.hash(), &private_key)?;
    transaction.append_signature(signature);
    if tx_validator::validate(&transaction, &public_key)? {}
    Ok(())
}

fn read_cli(command: &str) -> Result<(), RitCoinErrror> {
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
            let amount = send_parameters[2].parse::<usize>()?;
            send(recipient_address, amount)
        }
        _ => return Ok(()),
    }
}

pub fn cli() -> Result<(), RitCoinErrror> {
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
