use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(String),
    #[error("image error: {0}")]
    ImageError(String),
    #[error("lib error: {0}")]
    LibError(String),
}