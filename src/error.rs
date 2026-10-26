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

impl From<std::io::Error> for JeffPlironError {
    fn from(error: std::io::Error) -> Self {
        JeffPlironError::SerializationFailed(error.to_string())
    }
}

impl From<jeff::JeffError> for JeffPlironError {
    fn from(error: jeff::JeffError) -> Self {
        JeffPlironError::DeserializationFailed(error.to_string())
    }
}
