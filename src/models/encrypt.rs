use std::io;

use openssl::hash::{Hasher, MessageDigest};

pub struct Enrypt;

impl Enrypt {
    pub fn hash(word: &str) -> Result<[u8; 32], io::Error> {
        let mut hasher = Hasher::new(MessageDigest::sha256())?;
        hasher.update(word.as_bytes())?;
        let mut hashed: [u8; 32] = [0; 32];

        let _ = &hasher.finish_xof(&mut hashed)?;

        Ok(hashed)
    }
}
