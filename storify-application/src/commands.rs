use storify_domain::value_objects::UserName;

#[derive(Debug)]
pub struct CreateUser {
    user_name: UserName,
}

impl CreateUser {
    pub fn new(user_name: UserName) -> Self {
        Self { user_name }
    }

    pub fn user_name(self) -> UserName {
        self.user_name
    }
}

