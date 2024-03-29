use thiserror::Error;

mod types;

pub use types::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to serialize structure as XML")]
    Serialize(#[from] xml_struct::Error),

    #[error("failed to deserialize structure from XML")]
    Deserialize(#[from] quick_xml::DeError),

    #[error("error manipulating XML data")]
    Xml(#[from] quick_xml::Error),
}
