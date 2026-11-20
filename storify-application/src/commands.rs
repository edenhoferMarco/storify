use storify_domain::value_objects::{SpaceName, UserId, UserName};

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

