use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserNameError {
    #[error("UserName must not be empty! Input: '{0}'")]
    EmptyUserName(String),
}

#[derive(Error, Debug)]
pub enum SpaceNameError {
    #[error("SpaceName must not be empty! Input: '{0}'")]
    EmptySpaceName(String),
}

#[derive(Error, Debug)]
pub enum DirectoryNameError {
    #[error("DirectoryName must not be empty! Input: '{0}'")]
    EmptyDirectoryName(String),
}

#[derive(Error, Debug)]
pub enum SpaceError {
    #[error("Could not create root directory for Space: '{0}'!")]
    RootDirCreationFailed(String)
}

#[derive(Error, Debug)]
pub enum FileNameError {
    #[error("FileName must not be empty! Input: '{0}'")]
    EmptyFileName(String),
}