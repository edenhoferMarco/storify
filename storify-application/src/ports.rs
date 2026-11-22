use std::io::Bytes;
use storify_domain::entities::{Directory, StoredFile};
use crate::errors::FileSystemError;

pub trait FileSystem {
    fn store_file(&self, file: &StoredFile, location: &Directory, data: Bytes<u8>) -> Result<(), FileSystemError>;
    fn create_directory(&self, directory: &Directory, location: &Directory) -> Result<(), FileSystemError>;
    fn get_file(&self, file: &StoredFile, location: &Directory) -> Result<Bytes<u8>, FileSystemError>;
}