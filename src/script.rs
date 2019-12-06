use crate::errors::*;
use crate::opcodes::*;
use crate::wallet;
use std::ops::{Deref, DerefMut};

struct Stack(Vec<Vec<u8>>);

impl Deref for Stack {
    type Target = Vec<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Stack {
    fn new(sig_script: &[u8]) -> Self {
        let mut sig_script = sig_script.to_vec();
        //remove sig length
        sig_script.remove(0);
        //remove sig type
        sig_script.remove(71);
        //remove pub_key len
        sig_script.remove(71);
        Self(vec![
            sig_script[..sig_script.len() - 65].to_owned(),
            sig_script[sig_script.len() - 65..].to_owned(),
        ])
    }

    fn op_dup(&mut self) -> Result<(), RitCoinErrror<'static>> {
        if let Some(last) = self.last() {
            let last = last.clone();
            self.push(last);
            Ok(())
        } else {
            Err(RitCoinErrror::from("Stack must contain at least 1 element"))
        }
    }

    fn op_hash160(&mut self) -> Result<(), RitCoinErrror<'static>> {
        if let Some(last) = self.last() {
            let hash = wallet::pk_hash_from_public_key(last);
            self.push(hash);
            Ok(())
        } else {
            Err(RitCoinErrror::from("Stack must contain at least 1 element"))
        }
    }

    fn op_equalverify(&mut self) -> Result<(), RitCoinErrror<'static>> {
        match (self.pop(), self.pop()) {
            (Some(a), Some(b)) => {
                if a == b {
                    Ok(())
                } else {
                    Err(RitCoinErrror::from("pkhashes didn`t match"))
                }
            }
            _ => Err(RitCoinErrror::from("pkhash not found")),
        }
    }

    fn op_check_sig(&mut self, hash: &[u8]) -> Result<(), RitCoinErrror<'static>> {
        let public_key = self.pop();
        let signature = self.pop();
        match (public_key, signature) {
            (Some(public_key), Some(signature)) => {
                Ok(wallet::verify(hash, &public_key, &signature)?)
            }
            _ => Err(RitCoinErrror::from("Public key or signature not found!")),
        }
    }
}
//TODO More efficient and complete script
pub fn execute(
    sig_script: &[u8],
    script_pubkey: &[u8],
    hash: &[u8],
) -> Result<(), RitCoinErrror<'static>> {
    let mut stack = Stack::new(sig_script);
    let mut pk_hash = vec![];
    for opcode in script_pubkey {
        match *opcode {
            OP_DUP => stack.op_dup()?,
            OP_HASH160 => stack.op_hash160()?,
            OP_EQUALVERIFY => {
                stack.push(pk_hash.clone());
                stack.op_equalverify()?
            }
            OP_CHECKSIG => stack.op_check_sig(hash)?,
            opcode => pk_hash.push(opcode),
        }
    }
    Ok(())
}
