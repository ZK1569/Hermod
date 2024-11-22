use openssl::pkey::{PKey, Private};
use openssl::x509::X509;
use std::io::Write;
use std::{fs, io};

const FILE_NAME_CERT: &str = "/self_cert.cert";
const FILE_NAME_PVT_KEY: &str = "/pvt_key.pem";

pub fn save_certificate(cert: X509, path: &str) -> Result<(), io::Error> {
    let cert_bin = cert.to_pem()?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path.to_owned() + FILE_NAME_CERT)?;

    let _ = file.write_all(&cert_bin);

    Ok(())
}

pub fn save_pvt_key(key: PKey<Private>, path: &str) -> Result<(), io::Error> {
    let key_bin = key.private_key_to_pem_pkcs8()?;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path.to_owned() + FILE_NAME_PVT_KEY)?;

    let _ = file.write_all(&key_bin);

    Ok(())
}

pub fn read_self_certificate(path: &str) -> Result<X509, io::Error> {
    let contents_bin = fs::read_to_string(path.to_owned() + FILE_NAME_CERT)?;
    let cert = X509::from_pem(contents_bin.as_bytes())?;

    Ok(cert)
}

pub fn read_pvt_key(path: &str) -> Result<PKey<Private>, io::Error> {
    let contents_bin = fs::read_to_string(path.to_owned() + FILE_NAME_PVT_KEY)?;
    let key = PKey::private_key_from_pem(contents_bin.as_bytes())?;

    Ok(key)
}

pub fn read_certificate(path_to_file: &str) -> Result<X509, io::Error> {
    let contents_bin = fs::read_to_string(path_to_file)?;
    let cert = X509::from_pem(contents_bin.as_bytes())?;

    Ok(cert)
}

pub fn delete_certificate(path: &str) -> Result<(), io::Error> {
    let _ = fs::remove_file(path.to_owned() + FILE_NAME_CERT)?;
    Ok(())
}

pub fn delete_pvt_key(path: &str) -> Result<(), io::Error> {
    let _ = fs::remove_file(path.to_owned() + FILE_NAME_PVT_KEY);
    Ok(())
}
