use core::fmt;
use secp256k1::serde::Serialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use secp256k1::rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

#[derive(Debug, Clone)]
struct WriteError;

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to write keys to file")
    }
}

struct KeyPair {
    pub_key: PublicKey,
    priv_key: SecretKey,
}

impl KeyPair {
    pub fn generate(&mut self) {
        let secp = Secp256k1::new();
        let (priv_key, p_key) = secp.generate_keypair(&mut OsRng);
        self.pub_key = p_key;
        self.priv_key = priv_key;
    }

    pub fn write_keys(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(path_str) = path.to_str() {
            let mut key_file = File::open(path_str)?;

            let out = serde_json::to_vec(&self.priv_key)?;
            key_file.write_all(&out)?;
        }

        Ok(())
    }

    pub fn read_keys(&mut self, path: &Path) -> Result<(), std::io::Error> {
        let secp = Secp256k1::new();
        if let Some(path_str) = path.to_str() {
            let mut key_file = File::open(path_str)?;
            let mut raw_key: Vec<u8> = Vec::new();
            key_file.read_to_end(&mut raw_key)?;
            self.priv_key = serde_json::from_slice(&raw_key)?;
            self.pub_key = self.priv_key.public_key(&secp);
        }

        Ok(())
    }
}
