use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("No space left for storing file: '{0}!")]
    NoSpaceLeft(String),
}