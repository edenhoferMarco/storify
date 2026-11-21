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
pub enum FileNameError {
    #[error("FileName must not be empty! Input: '{0}'")]
    EmptyFileName(String),
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("The username '{0}' is already taken!")]
    UsernameAlreadyTaken(String),
}

#[derive(Error, Debug)]
pub enum SpaceError {
    #[error("An internal error happened! '{0}")]
    InternalError(String),
}

#[derive(Error, Debug)]
pub enum DirectoryError {
    #[error("An internal error happened! '{0}")]
    InternalError(String),
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("An internal error happened! '{0}")]
    InternalError(String),
}