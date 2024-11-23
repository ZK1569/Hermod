use std::io;

use openssl::x509::X509;
use reqwest::{multipart, StatusCode};

pub struct ServerApi {}

impl ServerApi {
    pub async fn signe_certificate(cert: &X509) -> Result<X509, Box<dyn std::error::Error>> {
        let cert_pem = cert.to_pem()?;

        let part = multipart::Part::bytes(cert_pem)
            .file_name("certificate.pem")
            .mime_str("application/x-pem-file")?;

        let form = multipart::Form::new().part("certificate", part);

        let client = reqwest::Client::new();

        let response = client
            .post("http://localhost:5001/signe")
            .multipart(form)
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let response_text = response.text().await?;
            let cert_signed = X509::from_pem(response_text.as_bytes())?;
            return Ok(cert_signed);
        } else {
            let error_message = format!(
                "API response error: Status Code: {}, Message: {}",
                response.status(),
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unable to read response body".to_string())
            );
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                error_message,
            )));
        }
    }

    pub async fn get_server_certificate() -> Result<X509, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .get("http://localhost:5001/certificate")
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let response_text = response.text().await?;
            let server_cert = X509::from_pem(response_text.as_bytes())?;
            return Ok(server_cert);
        } else {
            let error_message = format!(
                "API response error: Status Code: {}, Message: {}",
                response.status(),
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unable to read response body".to_string())
            );
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                error_message,
            )));
        }
    }
}
