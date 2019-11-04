use crate::RitCoinErrror;
use crate::wallet;
pub use std::fs::File;
pub use std::io::prelude::*;
use std::io::stdin;

const PATH_TO_ADDRESS: &str = "data/address.txt";
const FILE_NAME: &str = "data/privkey.txt";

pub fn new() -> Result<(), RitCoinErrror> {
    let (private_key, public_key) = wallet::generate_ecdsa_key_pair();
    let pub_address = wallet::get_address(&public_key)?;
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

fn read_cli(command: &str) -> Result<(), RitCoinErrror> {
    match command {
        "new" => new(),
        command if command.starts_with("import") => {
            let path = command.split_ascii_whitespace().collect::<Vec<&str>>()[1];
            import(path)
        }
        _ => return Ok(())
    }
}

pub fn cli() -> Result<(), RitCoinErrror> {
    loop {
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => read_cli(&buf.trim())?,
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
        }
    }
    Ok(())
}