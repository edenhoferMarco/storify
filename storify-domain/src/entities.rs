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
        let directories = Vec::new();
        let files = Vec::new();

        Self { id, name, owning_space_id, files, directories }
    }

    pub fn id(&self) -> &DirectoryId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // TODO: Not sure if space_id needed in the directory - check when application is more clear
    pub fn owning_space_id(&self) -> &SpaceId {
        &self.owning_space_id
    }

    pub fn directories(&self) -> &[DirectoryId] {
        &self.directories
    }

    pub fn files(&self) -> &[FileId] {
        &self.files
    }
}

pub struct Space {
    id: SpaceId,
    name: String,
    directory_id: DirectoryId,
    members: Vec<Member>,
}

impl Space {
    pub fn new(name: impl Into<String>, owning_user_id: UserId) -> Self {
        let name = name.into();
        let id = SpaceId::new();
        let owner = Member::new(owning_user_id, Permission::Own);
        let root_dir = Directory::new(id, format!("{name}_root"));
        let root_dir_id = *root_dir.id();
        let members: Vec<Member> = vec![owner];

        Self { id, name, directory_id:root_dir_id, members }
    }

    pub fn id(&self) -> SpaceId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn directory_id(&self) -> DirectoryId {
        self.directory_id
    }

    pub fn members(&self) -> &[Member] {
        &self.members
    }
}

pub struct StoredFile {
    id: FileId,
    name: String,
    directory_id: DirectoryId,
}

impl StoredFile {
    pub fn new(file_name: impl Into<String>, owning_directory_id: DirectoryId) -> Self {
        let id = FileId::new();
        let name = file_name.into();

        Self { id, name, directory_id: owning_directory_id }
    }

    pub fn id(&self) -> FileId {
        self.id
    }

    pub fn directory_id(&self) -> DirectoryId {
        self.directory_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}