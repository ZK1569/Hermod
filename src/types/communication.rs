use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Communication {
    CommunicationText(CommunicationText),
    CommunicationCertificate(CommunicationCertificate),
    CommunicationFile(CommunicationFile),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommunicationText {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommunicationCertificate {}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommunicationFile {}
