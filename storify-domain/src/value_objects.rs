// Default is suggested for our id structs like FileId - but a random generated uuid does not seem
// as a meaningful default. This is why Default is not implemented and this rule disabled for this.
#![allow(clippy::new_without_default)]

use uuid::Uuid;
use crate::errors::{DirectoryNameError, FileNameError, SpaceNameError, UserNameError};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FileId(Uuid);
impl FileId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DirectoryId(Uuid);

impl DirectoryId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SpaceId(Uuid);

impl SpaceId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
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
        
        Ok(Self(validated_name))
    }

    pub fn as_string(&self) -> &str {
        &self.0
    }

    fn validate_name(name: &str) -> Result<String, UserNameError> {
        if name.trim().is_empty() {
            return Err(UserNameError::EmptyUserName(name.to_owned()));
        }

        Ok(name.to_owned())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SpaceName(String);

impl SpaceName {
    pub fn new(name: impl Into<String>) -> Result<SpaceName, SpaceNameError> {
        let owned_name = name.into();
        let validated_name = Self::validate_name(&owned_name)?;

        Ok(Self(validated_name))
    }

    pub fn as_string(&self) -> &str {
        &self.0
    }

    fn validate_name(name: &str) -> Result<String, SpaceNameError> {
        if name.trim().is_empty() {
            return Err(SpaceNameError::EmptySpaceName(name.to_owned()));
        }

        Ok(name.to_owned())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DirectoryName(String);

impl DirectoryName {
    pub fn new(name: impl Into<String>) -> Result<DirectoryName, DirectoryNameError> {
        let owned_name = name.into();
        let validated_name = Self::validate_name(&owned_name)?;

        Ok(Self(validated_name))
    }

    pub fn as_string(&self) -> &str {
        &self.0
    }

    fn validate_name(name: &str) -> Result<String, DirectoryNameError> {
        if name.trim().is_empty() {
            return Err(DirectoryNameError::EmptyDirectoryName(name.to_owned()));
        }

        Ok(name.to_owned())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FileName(String);

impl FileName {
    pub fn new(name: impl Into<String>) -> Result<FileName, FileNameError> {
        let owned_name = name.into();
        let validated_name = Self::validate_name(&owned_name)?;

        Ok(Self(validated_name))
    }

    pub fn as_string(&self) -> &str {
        &self.0
    }

    fn validate_name(name: &str) -> Result<String, FileNameError> {
        if name.trim().is_empty() {
            return Err(FileNameError::EmptyFileName(name.to_owned()));
        }

        Ok(name.to_owned())
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Own
}