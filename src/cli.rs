use crate::errors::*;
use crate::miner_cli;
use crate::wallet_cli;
use crate::*;
pub use std::fs::{self, File};
pub use std::io::prelude::*;
use std::io::stdin;

fn read_cli(
    command: &str,
    prepared_transactions: &mut Vec<Vec<u8>>,
    ritcoin_state: Arc<RitCoinState>,
) -> Result<(), RitCoinErrror<'static>> {
    match command {
        "new" => wallet_cli::new(),
        command if command.starts_with("import") => {
            let path = command.split_ascii_whitespace().collect::<Vec<&str>>()[1];
            wallet_cli::import(path)
        }
        command if command.starts_with("send") => {
            let command = command.replace(',', "");
            let send_parameters = command.split_ascii_whitespace().collect::<Vec<&str>>();
            let recipient_address = send_parameters[1];
            let amount = send_parameters[2].parse::<u32>()?;
            wallet_cli::send(recipient_address, amount, prepared_transactions)
        }
        command if command.starts_with("broadcast") => {
            let serialized_tx = command.split_ascii_whitespace().collect::<Vec<&str>>()[1];
            wallet_cli::broadcast(serialized_tx, prepared_transactions)
        }
        command if command.starts_with("balance") => {
            let address = command.split_ascii_whitespace().collect::<Vec<&str>>()[1];
            wallet_cli::balance(address, ritcoin_state);
            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn cli(ritcoin_state: Arc<RitCoinState>) -> Result<(), RitCoinErrror<'static>> {
    let mut prepared_transactions: Vec<Vec<u8>> = vec![vec![]];
    loop {
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => read_cli(
                buf.trim(),
                &mut prepared_transactions,
                ritcoin_state.clone(),
            )?,
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
        }
    }
    Ok(())
}
