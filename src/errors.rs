#[derive(Debug)]
pub enum RitCoinErrror {
    IoError(std::io::Error),
    Base58Error(bs58::decode::Error),
    HexError(hex::FromHexError),
    Secp256k1Error(secp256k1::Error)
}

impl From<bs58::decode::Error> for RitCoinErrror {
    fn from(e: bs58::decode::Error) -> Self {
        RitCoinErrror::Base58Error(e)
    }
}

impl From<std::io::Error> for RitCoinErrror {
    fn from(e: std::io::Error) -> Self {
        RitCoinErrror::IoError(e)
    }
}

impl From<hex::FromHexError> for RitCoinErrror {
    fn from(e: hex::FromHexError) -> Self {
        RitCoinErrror::HexError(e)
    }
}

impl From<secp256k1::Error> for RitCoinErrror {
    fn from(e: secp256k1::Error) -> Self {
        RitCoinErrror::Secp256k1Error(e)
    }
}