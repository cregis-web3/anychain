use crate::{BitcoinFormat, BitcoinNetwork, Prefix};
use anychain_core::no_std::*;
use anychain_core::{AddressError, Network, NetworkError};

use core::{fmt, str::FromStr};
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Bitcoin;

impl Network for Bitcoin {
    const NAME: &'static str = "bitcoin";
}

impl BitcoinNetwork for Bitcoin {
    /// Returns the address prefix of the given network.
    fn to_address_prefix(format: BitcoinFormat) -> Prefix {
        match format {
            BitcoinFormat::P2PKH => Prefix::Version(0x00),
            BitcoinFormat::P2WSH => Prefix::Version(0x00),
            BitcoinFormat::P2SH_P2WPKH => Prefix::Version(0x05),
            BitcoinFormat::Bech32 => Prefix::AddressPrefix("bc".to_string()),
            f => panic!("{} does not support address format {}", Self::NAME, f),
        }
    }

    /// Returns the network of the given address prefix.
    fn from_address_prefix(prefix: Prefix) -> Result<Self, AddressError> {
        match prefix {
            Prefix::Version(version) => match version {
                0x00 | 0x05 => Ok(Self),
                _ => Err(AddressError::Message(format!(
                    "Invalid version byte {:#0x} for network {}",
                    version,
                    Self::NAME,
                ))),
            },
            Prefix::AddressPrefix(prefix) => match prefix.as_str() {
                "bc" => Ok(Self),
                _ => Err(AddressError::Message(format!(
                    "Invalid Bech32 prefix for network {}",
                    Self::NAME,
                ))),
            },
        }
    }
}

impl FromStr for Bitcoin {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::NAME => Ok(Self),
            _ => Err(NetworkError::InvalidNetwork(s.into())),
        }
    }
}

impl fmt::Display for Bitcoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
