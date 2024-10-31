use std::io;

use openssl::{
    hash::{Hasher, MessageDigest},
    pkcs5::pbkdf2_hmac,
    symm::{Cipher, Crypter, Mode},
};

pub struct Encrypt;

impl Encrypt {
    pub fn hash(word: &str) -> Result<[u8; 32], io::Error> {
        let mut hasher = Hasher::new(MessageDigest::sha256())?;
        hasher.update(word.as_bytes())?;
        let hashed = hasher.finish()?;

        let mut result = [0u8; 32];
        if hashed.len() == 32 {
            result.copy_from_slice(&hashed);
            Ok(result)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Unexpected hash length",
            ))
        }
    }

    pub fn derive_key_from_password(password: &str, iterations: usize) -> [u8; 32] {
        let mut key: [u8; 32] = [0; 32];
        pbkdf2_hmac(
            password.as_bytes(),
            b"this_is_the_salt",
            iterations,
            MessageDigest::sha256(),
            &mut key,
        )
        .expect("Key derivation failed");
        key
    }

    pub fn encrypt_message(message: &[u8], key: &[u8; 32]) -> Vec<u8> {
        let cipher = Cipher::aes_256_cbc();
        let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key, None)
            .expect("Erreur d'initialisation du chiffreur");

        let mut ciphertext = vec![0; message.len() + cipher.block_size()];
        let mut count = encrypter
            .update(message, &mut ciphertext)
            .expect("Erreur lors du chiffrement");
        count += encrypter
            .finalize(&mut ciphertext[count..])
            .expect("Erreur lors de la finalisation");

        ciphertext.truncate(count);
        ciphertext
    }

    pub fn decrypt_message(ciphertext: &[u8], key: &[u8; 32]) -> Vec<u8> {
        let cipher = Cipher::aes_256_cbc();
        let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key, None)
            .expect("Erreur d'initialisation du déchiffreur");

        let mut plaintext = vec![0; ciphertext.len() + cipher.block_size()];
        let mut count = decrypter
            .update(ciphertext, &mut plaintext)
            .expect("Erreur lors du déchiffrement");
        count += decrypter
            .finalize(&mut plaintext[count..])
            .expect("Erreur lors de la finalisation");

        plaintext.truncate(count);
        plaintext
    }
}
