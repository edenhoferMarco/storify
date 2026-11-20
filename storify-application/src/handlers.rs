use crate::commands::CreateUser;
use storify_domain::entities::User;
use storify_domain::value_objects::UserId;

pub struct UserHandler {

}

impl UserHandler {
    pub fn receive(command: CreateUser) -> UserId {
        println!("Received command: {:?}", command);
        let new_user = User::new(command.user_name());
        new_user.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use storify_domain::value_objects::UserName;
    #[test]
    fn test_receive() -> Result<(), Box<dyn std::error::Error>> {
        let user_name = UserName::new("Der Testboy")?;
        let command = CreateUser::new(user_name);

        UserHandler::receive(command);

        Ok(())
    }
}