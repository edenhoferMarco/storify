use uuid::Uuid;
use crate::errors::UserNameError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileId(Uuid);

impl FileId {
    pub fn new() -> Self {
        Self (Uuid::now_v7())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DirectoryId(Uuid);

impl DirectoryId {
    pub fn new() -> Self {
        Self (Uuid::now_v7())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SpaceId(Uuid);

impl SpaceId {
    pub fn new() -> Self {
        Self (Uuid::now_v7())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self (Uuid::now_v7())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: impl Into<String>) -> Result<UserName, UserNameError> {
        let owned_name = name.into();
        let validated_name = Self::validate_name(&owned_name)?;
        
        Ok(Self (validated_name))
    }

    pub fn as_string(&self) -> &str {
        &self.0
    }

    fn validate_name(name: &str) -> Result<String, UserNameError> {
        if name.trim().is_empty() {
            return Err(UserNameError::InvalidUsername(name.to_owned()));
        }

        Ok(name.to_owned())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Own
}