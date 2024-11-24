use std::io;

use openssl::{
    asn1::Asn1Time,
    bn::{BigNum, MsbOption},
    error::ErrorStack,
    hash::{Hasher, MessageDigest},
    pkcs5::pbkdf2_hmac,
    pkey::{PKey, Private},
    rsa::Rsa,
    symm::{Cipher, Crypter, Mode},
    x509::{
        extension::{BasicConstraints, KeyUsage, SubjectKeyIdentifier},
        X509NameBuilder, X509,
    },
};

pub struct Encrypt;

impl Encrypt {
    pub fn hash(word: &str) -> Result<[u8; 32], io::Error> {
        // FIX: Change sha256 to sha512
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

    pub fn mk_ca_cert(
        username: &str,
        email: &str,
        country: &str,
        locality: &str,
    ) -> Result<(X509, PKey<Private>), ErrorStack> {
        let rsa = Rsa::generate(2048)?;
        let key_pair = PKey::from_rsa(rsa)?;

        let mut x509_name = X509NameBuilder::new()?;
        x509_name.append_entry_by_text("C", country)?;
        x509_name.append_entry_by_text("ST", locality)?;
        x509_name.append_entry_by_text("O", "UQAC")?;
        x509_name.append_entry_by_text("CN", username)?;
        x509_name.append_entry_by_text("emailAddress", email)?;
        let x509_name = x509_name.build();

        let mut cert_builder = X509::builder()?;
        cert_builder.set_version(2)?;
        let serial_number = {
            let mut serial = BigNum::new()?;
            serial.rand(159, MsbOption::MAYBE_ZERO, false)?;
            serial.to_asn1_integer()?
        };
        cert_builder.set_serial_number(&serial_number)?;
        cert_builder.set_subject_name(&x509_name)?;
        cert_builder.set_issuer_name(&x509_name)?;
        cert_builder.set_pubkey(&key_pair)?;
        let not_before = Asn1Time::days_from_now(0)?;
        cert_builder.set_not_before(&not_before)?;
        let not_after = Asn1Time::days_from_now(365)?;
        cert_builder.set_not_after(&not_after)?;

        cert_builder.append_extension(BasicConstraints::new().critical().ca().build()?)?;
        cert_builder.append_extension(
            KeyUsage::new()
                .critical()
                .key_cert_sign()
                .crl_sign()
                .build()?,
        )?;

        let subject_key_identifier =
            SubjectKeyIdentifier::new().build(&cert_builder.x509v3_context(None, None))?;
        cert_builder.append_extension(subject_key_identifier)?;

        cert_builder.sign(&key_pair, MessageDigest::sha256())?;
        let cert = cert_builder.build();

        Ok((cert, key_pair))
    }

    pub fn certificate_check_signature(server_cert: &X509, client_cert: &X509) -> bool {
        let server_public_key = server_cert.public_key().unwrap();
        client_cert.verify(&server_public_key).unwrap()
    }
}
