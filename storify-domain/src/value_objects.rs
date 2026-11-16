use uuid::Uuid;

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
    pub fn new(name: &str) -> Self {
        Self (name.to_owned())
    }

    pub fn as_string(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Own
}