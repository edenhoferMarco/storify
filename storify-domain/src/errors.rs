use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserNameError {
    #[error("'{0}' is not a valid user name!")]
    InvalidUsername(String),
}