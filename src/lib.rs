use thiserror::Error;

mod types;

pub use types::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to serialize structure as XML")]
    Serialize {
        #[from]
        source: xml_struct::Error,
    },

    #[error("failed to deserialize structure from XML")]
    Deserialize {
        #[from]
        source: quick_xml::DeError,
    },

    #[error("error manipulating XML data")]
    Xml {
        #[from]
        source: quick_xml::Error,
    },
}
