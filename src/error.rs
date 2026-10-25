use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum JeffPlironError {
    #[error("unknown module symbol: {0}")]
    UnknownSymbol(String),
    #[error("missing string entry: {0}")]
    MissingString(String),
    #[error("invalid value mapping: {0}")]
    InvalidValueMapping(String),
    #[error("unsupported JEFF op kind: {0}")]
    UnsupportedOp(String),
    #[error("serialization failed: {0}")]
    SerializationFailed(String),
    #[error("deserialization failed: {0}")]
    DeserializationFailed(String),
}

pub type JeffResult<T> = Result<T, JeffPlironError>;
