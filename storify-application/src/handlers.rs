use crate::commands::{CreateDirectory, CreateSpace, CreateUser, StoreFile};
use storify_domain::entities::{Directory, Space, StoredFile, User};
use storify_domain::value_objects::{DirectoryId, FileId, SpaceId, UserId};

pub struct UserHandler {

}

impl UserHandler {
    pub fn receive(command: CreateUser) -> UserId {
        println!("Received command: {:?}", command);
        let CreateUser { user_name } = command;
        let new_user = User::new(user_name);

        new_user.id()
    }
}

pub struct SpaceHandler {

}

impl SpaceHandler {
    pub fn receive(command: CreateSpace) -> SpaceId {
        println!("Received command: {:?}", command);
        let CreateSpace { space_name, user_id } = command;
        let new_space = Space::new(space_name, user_id);

        new_space.id()
    }
}

pub struct DirectoryHandler {

}

impl DirectoryHandler {
    pub fn receive(command: CreateDirectory) -> DirectoryId {
        println!("Received command: {:?}", command);
        let CreateDirectory { directory_name, owning_space_id } = command;
        let new_directory = Directory::new(owning_space_id, directory_name);

        new_directory.id()
    }
}

pub struct FileHandler {

}

impl FileHandler {
    pub fn receive(command: StoreFile) -> FileId {
        println!("Received command: {:?}", command);
        let StoreFile { owning_directory_id, file_name } = command;
        let new_file = StoredFile::new(file_name, owning_directory_id);

        new_file.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use storify_domain::value_objects::{DirectoryName, FileName, SpaceName, UserName};
    #[test]
    fn test_receive_create_user() -> Result<(), Box<dyn std::error::Error>> {
        let user_name = UserName::new("Testy Mc Testboy")?;
        let create_user_command = CreateUser::new(user_name);

        UserHandler::receive(create_user_command);

        Ok(())
    }

    #[test]
    fn test_receive_create_space() -> Result<(), Box<dyn std::error::Error>> {
        let user_name = UserName::new("Testy Mc Testboy")?;
        let user = User::new(user_name);
        let user_id = user.id();
        let space_name = SpaceName::new("Testies awesome testspace")?;
        let create_space_command = CreateSpace::new(space_name, user_id);

        SpaceHandler::receive(create_space_command);

        Ok(())
    }

    #[test]
    fn test_receive_create_directory() -> Result<(), Box<dyn std::error::Error>> {
        let user_name = UserName::new("Testy Mc Testboy")?;
        let user = User::new(user_name);
        let user_id = user.id();
        let space_name = SpaceName::new("Testies awesome testspace")?;
        let space = Space::new(space_name, user_id);
        let dir_name = DirectoryName::new("Testies fancy new directory")?;
        let create_dir_command = CreateDirectory::new(space.id(), dir_name);

        DirectoryHandler::receive(create_dir_command);

        Ok(())
    }

    #[test]
    fn test_receive_store_file() -> Result<(), Box<dyn std::error::Error>> {
        let user_name = UserName::new("Testy Mc Testboy")?;
        let user = User::new(user_name);
        let user_id = user.id();
        let space_name = SpaceName::new("Testies awesome testspace")?;
        let space = Space::new(space_name, user_id);
        let file_name = FileName::new("Testies terrific file")?;
        let store_file_command = StoreFile::new(space.directory_id(), file_name);

        FileHandler::receive(store_file_command);

        Ok(())
    }
}