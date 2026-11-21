use std::fs::File;
use crate::commands::{CreateDirectory, CreateSpace, CreateUser, StoreFile};
use crate::entities::{Directory, Space, User};
use crate::errors::{DirectoryError, FileError, SpaceError, UserError};
use crate::value_objects::{SpaceId, UserId, UserName};

pub trait UserRepository {
    fn create_user(&self, command: CreateUser) -> Result<User, UserError>;
    fn get_user_by_id(&self, user_id: UserId) -> Result<Option<User>, UserError>;
    fn get_all_users(&self) -> Result<Vec<User>, UserError>;
    fn save_user(&self, user: &User) -> Result<(), UserError>;
}

pub trait SpaceRepository {
    fn create_space(&self, command: CreateSpace) -> Result<Space, SpaceError>;
    fn get_space_by_id(&self, space_id: SpaceId) -> Result<Option<Space>, SpaceError>;
    fn get_all_spaces(&self) -> Result<Vec<Space>, SpaceError>;
}

pub trait DirectoryRepository {
    fn create_directory(&self, command: CreateDirectory) -> Result<Directory, DirectoryError>;
}

pub trait FileRepository {
    fn create_file(&self, command: StoreFile) -> Result<File, FileError>;
}