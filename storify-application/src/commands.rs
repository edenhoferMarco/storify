use storify_domain::value_objects::{DirectoryId, DirectoryName, FileId, FileName, Permission, SpaceId, SpaceName, UserId, UserName};

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
pub struct DeleteUser {
    pub user_id: UserId,
}

impl DeleteUser {
    pub fn new(user_id: UserId) -> Self {
        Self { user_id }
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
pub struct DeleteSpace {
    pub space_id: SpaceId,
}

impl DeleteSpace {
    pub fn new(space_id: SpaceId) -> Self {
        Self { space_id }
    }
}

#[derive(Debug)]
pub struct TransferOwnership {
    pub space_id: SpaceId,
    pub current_owner: UserId,
    pub new_owner: UserId,
}

impl TransferOwnership {
    pub fn new(space_id: SpaceId, current_owner: UserId, new_owner: UserId) -> Self {
        Self { space_id, current_owner, new_owner }
    }
}

#[derive(Debug)]
pub struct AddMember {
    pub space_id: SpaceId,
    pub new_member_id: UserId,
    pub permission: Permission
}

impl AddMember {
    pub fn new(space_id: SpaceId, new_member_id: UserId, permission: Permission) -> Self {
        Self { space_id, new_member_id, permission }
    }
}

#[derive(Debug)]
pub struct RemoveMember {
    pub space_id: SpaceId,
    pub member_to_remove_id: UserId,
}

impl RemoveMember {
    pub fn new(space_id: SpaceId, member_to_remove_id: UserId) -> Self {
        Self { space_id, member_to_remove_id }
    }
}

#[derive(Debug)]
pub struct ChangePermission {
    pub space_id: SpaceId,
    pub member_id: UserId,
    pub new_permission: Permission
}

impl ChangePermission {
    pub fn new(space_id: SpaceId, member_id: UserId, new_permission: Permission) -> Self {
        Self { space_id, member_id, new_permission }
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
pub struct DeleteDirectory {
    pub directory_id: DirectoryId,
}

impl DeleteDirectory {
    pub fn new(directory_id: DirectoryId) -> Self {
        Self { directory_id }
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

#[derive(Debug)]
pub struct DeleteFile {
    pub file_id: FileId,
}

impl DeleteFile {
    pub fn new(file_id: FileId) -> Self {
        Self { file_id }
    }
}