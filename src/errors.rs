#[derive(Debug)]
pub enum RitCoinErrror<'a> {
    IoError(std::io::Error),
    Base58Error(bs58::decode::Error),
    HexError(hex::FromHexError),
    Secp256k1Error(secp256k1::Error),
    ParseIntError(std::num::ParseIntError),
    BincodeError(bincode::Error),
    StrError(&'a str),
}

impl<'a> From<bs58::decode::Error> for RitCoinErrror<'a> {
    fn from(e: bs58::decode::Error) -> Self {
        RitCoinErrror::Base58Error(e)
    }
}

impl<'a> From<std::io::Error> for RitCoinErrror<'a> {
    fn from(e: std::io::Error) -> Self {
        RitCoinErrror::IoError(e)
    }
}

impl<'a> From<hex::FromHexError> for RitCoinErrror<'a> {
    fn from(e: hex::FromHexError) -> Self {
        RitCoinErrror::HexError(e)
    }
}

impl<'a> From<secp256k1::Error> for RitCoinErrror<'a> {
    fn from(e: secp256k1::Error) -> Self {
        RitCoinErrror::Secp256k1Error(e)
    }
}

impl<'a> From<std::num::ParseIntError> for RitCoinErrror<'a> {
    fn from(e: std::num::ParseIntError) -> Self {
        RitCoinErrror::ParseIntError(e)
    }
}

impl<'a> From<bincode::Error> for RitCoinErrror<'a> {
    fn from(e: bincode::Error) -> Self {
        RitCoinErrror::BincodeError(e)
    }
}

impl<'a> From<&'a str> for RitCoinErrror<'a> {
    fn from(e: &'a str) -> Self {
        RitCoinErrror::StrError(e)
    }
}
