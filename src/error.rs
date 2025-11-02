use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModerationError {
    #[error("http error: {0}")]
    Http(String),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("signing error: {0}")]
    Signing(String),

    #[error("other: {0}")]
    Other(String),
}

impl From<anyhow::Error> for ModerationError {
    fn from(e: anyhow::Error) -> Self {
        ModerationError::Other(e.to_string())
    }
}
