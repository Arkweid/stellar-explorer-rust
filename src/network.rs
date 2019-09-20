#![allow(dead_code)]

use crate::config::CONFIG;
use sha2::digest::Digest;
use sha2::Sha256;

/// A Stellar Network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Network {
    passphrase: String,
}

impl Network {
    /// Create new network with `passphrase`.
    pub fn new(passphrase: String) -> Network {
        Network { passphrase }
    }

    pub fn network() -> Network {
        match CONFIG.network().as_str() {
            "test" => Self::test_network(),
            "public" => Self::public_network(),
            _ => unreachable!(
                "Unknown network. Check config file, network key should be public or test"
            ),
        }
    }

    /// Create new network with the same passphrase as SDF public network.
    pub fn public_network() -> Network {
        Self::new(CONFIG.public_passphrase().to_owned())
    }

    /// Create new network with the same passphrase as SDF test network.
    pub fn test_network() -> Network {
        Self::new(CONFIG.test_passphrase().to_owned())
    }

    /// Return the network passphrase.
    pub fn passphrase(&self) -> &str {
        &self.passphrase
    }

    /// Return the network id, which is the hash of the network passphrase.
    pub fn network_id(&self) -> Vec<u8> {
        Sha256::digest(self.passphrase.as_bytes()).to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::Network;

    #[test]
    fn test_public_network_id() {
        let network = Network::public_network();
        let id = network.network_id();
        let expected_id = vec![
            0x7A, 0xC3, 0x39, 0x97, 0x54, 0x4E, 0x31, 0x75, 0xD2, 0x66, 0xBD, 0x02, 0x24, 0x39,
            0xB2, 0x2C, 0xDB, 0x16, 0x50, 0x8C, 0x01, 0x16, 0x3F, 0x26, 0xE5, 0xCB, 0x2A, 0x3E,
            0x10, 0x45, 0xA9, 0x79,
        ];
        assert_eq!(id, expected_id);
    }

    #[test]
    fn test_test_network_id() {
        let network = Network::test_network();
        let id = network.network_id();
        let expected_id = vec![
            0xCE, 0xE0, 0x30, 0x2D, 0x59, 0x84, 0x4D, 0x32, 0xBD, 0xCA, 0x91, 0x5C, 0x82, 0x03,
            0xDD, 0x44, 0xB3, 0x3F, 0xBB, 0x7E, 0xDC, 0x19, 0x05, 0x1E, 0xA3, 0x7A, 0xBE, 0xDF,
            0x28, 0xEC, 0xD4, 0x72,
        ];
        assert_eq!(id, expected_id);
    }
}
