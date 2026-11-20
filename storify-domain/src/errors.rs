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