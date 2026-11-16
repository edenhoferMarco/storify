use crate::errors::UserNameError;
use crate::value_objects::{UserId, UserName};

pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(name: impl Into<String>) -> Result<User, UserNameError> {
        Ok(Self { id: UserId::new(), name: UserName::new(name)? })
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }
}