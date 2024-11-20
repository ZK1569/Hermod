use openssl::pkey::{PKey, Private};
use openssl::x509::X509;
use std::fs::create_dir_all;
use std::io::Write;
use std::{fs, io};

pub fn certificate(cert: X509) -> Result<(), io::Error> {
    let _ = make_dir("/Users/zk/.config/hermod")?;

    let cert_bin = cert.to_pem()?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("/Users/zk/.config/hermod/self_cert.cert")?;

    let _ = file.write_all(&cert_bin);

    Ok(())
}

pub fn pvt_key(key: PKey<Private>) -> Result<(), io::Error> {
    let _ = make_dir("/Users/zk/.config/hermod")?;

    let key_bin = key.private_key_to_pem_pkcs8()?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("/Users/zk/.config/hermod/pvt_key.pem")?;

    let _ = file.write_all(&key_bin);

    Ok(())
}

pub fn make_dir(path: &str) -> Result<(), io::Error> {
    create_dir_all(path)
}
