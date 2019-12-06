use crate::errors::*;
use crate::miner_cli;
use crate::wallet_cli;
use crate::*;
pub use std::fs::{self, File};
pub use std::io::prelude::*;
use std::io::stdin;

pub const ADDRESS_PATH: &str = "data/address.txt";
pub const PRIVATE_KEY_PATH: &str = "data/private_key.txt";

fn read_cli(
    command: &str,
    prepared_transactions: &mut Vec<Vec<u8>>,
    ritcoin_state: Arc<RitCoinState>,
) -> Result<(), RitCoinErrror<'static>> {
    match command {
        "new -m" => miner_cli::new(),
        "new" => wallet_cli::new(ADDRESS_PATH),
        command if command.starts_with("import -m") => {
            let path = command.split_ascii_whitespace().collect::<Vec<&str>>()[2];
            miner_cli::import(path)
        }
        command if command.starts_with("import") => {
            let path = command.split_ascii_whitespace().collect::<Vec<&str>>()[1];
            wallet_cli::import(path, ADDRESS_PATH)
        }
        command if command.starts_with("send") => {
            let command = command.replace(',', "");
            let send_parameters = command.split_ascii_whitespace().collect::<Vec<&str>>();
            let recipient_address = send_parameters[1];
            let amount = send_parameters[2].parse::<u64>()?;
            wallet_cli::send(
                recipient_address,
                amount,
                prepared_transactions,
                ritcoin_state,
            )
        }
        command if command.starts_with("broadcast") => {
            let broadcast_parameters = command.split_ascii_whitespace().collect::<Vec<&str>>();
            let serialized_tx = broadcast_parameters[1];
            match broadcast_parameters.get(2) {
                Some(flag) if *flag == "-t" => {
                    wallet_cli::broadcast(serialized_tx, prepared_transactions, true)
                }
                _ => wallet_cli::broadcast(serialized_tx, prepared_transactions, false),
            }
        }
        command if command.starts_with("balance") => {
            let address = command.split_ascii_whitespace().collect::<Vec<&str>>()[1];
            wallet_cli::balance(address, ritcoin_state)?;
            Ok(())
        }
        command if command.starts_with("add node") => {
            let node = command.split_ascii_whitespace().collect::<Vec<&str>>()[2];
            miner_cli::add_node(node, ritcoin_state)
        }
        "mine" => miner_cli::mine(ritcoin_state),
        "consensus" => miner_cli::consensus(ritcoin_state),
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
