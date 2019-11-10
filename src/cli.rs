use crate::errors::*;
use crate::miner_cli;
use crate::wallet_cli;
pub use std::fs::{self, File};
pub use std::io::prelude::*;
use std::io::stdin;

fn read_cli(command: &str) -> Result<(), RitCoinErrror<'static>> {
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
            wallet_cli::send(recipient_address, amount)
        }
        command if command.starts_with("broadcast") => {
            let tx = command.split_ascii_whitespace().collect::<Vec<&str>>()[1];
            wallet_cli::broadcast(tx)
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
