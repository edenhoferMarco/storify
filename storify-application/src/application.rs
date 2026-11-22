use storify_domain::commands::CreateUser;
use storify_domain::entities::User;
use storify_domain::errors::UserError;
use storify_domain::ports::UserRepository;
use storify_domain::value_objects::{UserId, UserName};
use crate::ports::FileSystem;

pub struct Application {
    user_repo: Box<dyn UserRepository>,
    file_system: Box<dyn FileSystem>,
}

impl Application {
    pub fn new(user_repo: Box<dyn UserRepository>, file_system: Box<dyn FileSystem>) -> Self {
        Self { user_repo, file_system }
    }

    pub fn create_user(&self, command: CreateUser) -> Result<UserId, UserError> {
        println!("Received command: {:?}", command);
        let CreateUser { user_name } = command;
        let new_user = User::new(user_name);
        self.user_repo.save_user(&new_user)?;

        Ok(new_user.id())
    }
}