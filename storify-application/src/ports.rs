use storify_domain::entities::{Directory, StoredFile};
use crate::errors::FileSystemError;

pub trait FileSystem {
    fn store_file(file: &StoredFile, location: &Directory) -> Result<(), FileSystemError>;
    fn create_directory(directory: &Directory, location: &Directory) -> Result<(), FileSystemError>;
}