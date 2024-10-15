use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Clone, Hash)]
pub struct UserInfo {
    pub code: String,
    pub id: String,
    pub pwd: String,
    pub login_state: bool,
}

impl UserInfo {
    fn from_username(username: String) -> Self {
        //let mut hasher = DefaultHasher::new();
        //username.hash(&mut hasher);
        UserInfo {
            code: String::new(),
            id: String::new(),
            pwd: String::new(),
            login_state: false,
        }
    }

    fn get_code(&self) -> &str {
        self.code.as_str()
    }
}
