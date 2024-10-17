use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Clone, Hash, Debug)]
pub struct UserInfo {
    pub name: String,
    pub code: String,
    pub id: String,
    pub pwd: String,
    pub login_state: bool,
}

impl UserInfo {
    pub fn from_username(username: String) -> Self {
        //let mut hasher = DefaultHasher::new();
        //username.hash(&mut hasher);
        UserInfo {
            name: username,
            code: String::new(),
            id: String::new(),
            pwd: String::new(),
            login_state: false,
        }
    }

    pub fn get_code(&self) -> &str {
        self.code.as_str()
    }
}
