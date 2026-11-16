use crate::errors::UserNameError;
use crate::value_objects::{DirectoryId, FileId, Permission, SpaceId, UserId, UserName};

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

pub struct Member {
    user_id: UserId,
    permission: Permission,
}

impl Member {
    pub fn new(user_id: UserId, permission: Permission) -> Self {
        Self { user_id, permission }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn permission(&self) -> &Permission {
        &self.permission
    }
}

pub struct Directory {
    id: DirectoryId,
    name: String,
    owning_space_id: SpaceId,
    files: Vec<FileId>,
    directories: Vec<DirectoryId>,
}

impl Directory {
    pub fn new(owning_space_id: SpaceId, name: impl Into<String>) -> Self {
        let name = name.into();
        let id = DirectoryId::new();
        let directories: Vec<DirectoryId> = Vec::new();
        let files: Vec<FileId> = Vec::new();

        Self { id, name, owning_space_id, files, directories }
    }

    pub fn id(&self) -> &DirectoryId {
        &self.id
    }
}

pub struct Space {
    id: SpaceId,
    name: String,
    directory_id: DirectoryId,
    members: Vec<Member>,
}

impl Space {
    pub fn new(name: impl Into<String>, owning_user: UserId) -> Self {
        let name = name.into();
        let id = SpaceId::new();
        let owner = Member::new(owning_user, Permission::Own);
        let root_dir = Directory::new(id.clone(), format!("{name}_root"));
        let root_dir_id = root_dir.id().clone();
        let members: Vec<Member> = vec![owner];

        Self {
            id, name, directory_id:root_dir_id, members
        }
    }
}