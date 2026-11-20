use storify_domain::value_objects::{DirectoryId, DirectoryName, FileName, SpaceId, SpaceName, UserId, UserName};

#[derive(Debug)]
pub struct CreateUser {
    pub user_name: UserName,
}

impl CreateUser {
    pub fn new(user_name: UserName) -> Self {
        Self { user_name }
    }
}

#[derive(Debug)]
pub struct CreateSpace {
    pub space_name: SpaceName,
    pub user_id: UserId,
}

impl CreateSpace {
    pub fn new(space_name: SpaceName, user_id: UserId) -> Self {
        Self { space_name, user_id }
    }
}

#[derive(Debug)]
pub struct CreateDirectory {
    pub owning_space_id: SpaceId,
    pub directory_name: DirectoryName,
}

impl CreateDirectory {
    pub fn new(owning_space_id: SpaceId, directory_name: DirectoryName) -> Self {
        Self { owning_space_id, directory_name }
    }
}

#[derive(Debug)]
pub struct StoreFile {
    pub owning_directory_id: DirectoryId,
    pub file_name: FileName,
}

impl StoreFile {
    pub fn new(owning_directory_id: DirectoryId, file_name: FileName) -> Self {
        Self { owning_directory_id, file_name }
    }
}

