use thiserror::Error;
use crate::via_decoder;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Failed to parse JSON: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("An unknown error has occurred")]
    Unknown,

    #[error("Via decoder error: {0}")]
    ViaDecoder(#[from] via_decoder::Error),

    #[error("XML parsing error: {0}")]
    Xml(#[from] quick_xml::Error),

    #[error("UTF8 parsing error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("XML deserialization error: {0}")]
    XmlDe(#[from] quick_xml::de::DeError),
}