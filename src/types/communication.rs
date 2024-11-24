use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Communication {
    CommunicationText(CommunicationText),
    CommunicationCertificate(CommunicationCertificate),
    CommunicationFile(CommunicationFile),
    CommunicationPassword(CommunicationPassword),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommunicationText {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommunicationCertificate {
    pub certificate_state: CertificateState,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommunicationFile {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommunicationPassword {
    pub password_state: PasswordState,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum PasswordState {
    Submition,
    Correct,
    Incorrect,
    Failed,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CertificateState {
    Submition,
    Correct,
    Incorrect,
}
