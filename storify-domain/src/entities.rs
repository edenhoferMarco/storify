use crate::value_objects::{DirectoryId, DirectoryName, FileId, Permission, SpaceId, SpaceName, UserId, UserName};

pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(name: UserName) -> User {
        Self { id: UserId::new(), name }
    }

    pub fn id(&self) -> UserId {
        self.id
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

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn permission(&self) -> Permission {
        self.permission
    }
}

pub struct Directory {
    id: DirectoryId,
    name: DirectoryName,
    owning_space_id: SpaceId,
    files: Vec<FileId>,
    directories: Vec<DirectoryId>,
}

impl Directory {
    pub fn new(owning_space_id: SpaceId, name: DirectoryName) -> Self {
        let id = DirectoryId::new();
        let directories = Vec::new();
        let files = Vec::new();

        Self { id, name, owning_space_id, files, directories }
    }

    pub fn id(&self) -> DirectoryId {
        self.id
    }

    pub fn name(&self) -> &DirectoryName {
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
    name: SpaceName,
    directory_id: DirectoryId,
    members: Vec<Member>,
}

impl Space {
    pub fn new(name: SpaceName, owning_user_id: UserId) -> Self {
        let id = SpaceId::new();
        let owner = Member::new(owning_user_id, Permission::Own);
        let name_as_string = name.as_string();
        let root_dir_name = DirectoryName::new(format!("{name_as_string}_root"))
            .expect("Root directory name cannot be empty if SpaceName is valid!");
        let root_dir = Directory::new(id, root_dir_name);
        let root_dir_id = root_dir.id();
        let members: Vec<Member> = vec![owner];

        Self { id, name, directory_id:root_dir_id, members }
    }

    pub fn id(&self) -> SpaceId {
        self.id
    }

    pub fn name(&self) -> &SpaceName {
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