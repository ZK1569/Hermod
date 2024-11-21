use openssl::pkey::{PKey, Private};
use openssl::x509::X509;
use std::io::Write;
use std::{fs, io};

pub fn save_certificate(cert: X509, path: &str) -> Result<(), io::Error> {
    let cert_bin = cert.to_pem()?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path.to_owned() + "/.config/hermod/self_cert.cert")?;

    let _ = file.write_all(&cert_bin);

    Ok(())
}

pub fn save_pvt_key(key: PKey<Private>, path: &str) -> Result<(), io::Error> {
    let key_bin = key.private_key_to_pem_pkcs8()?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path.to_owned() + "/.config/hermod/pvt_key.pem")?;

    let _ = file.write_all(&key_bin);

    Ok(())
}
