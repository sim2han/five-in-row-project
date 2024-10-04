use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Clone, Hash)]
struct UserInfo {
    hash: String,
}

impl UserInfo {
    fn from_username(username: String) -> Self {
        let mut hasher = DefaultHasher::new();
        username.hash(&mut hasher);
        UserInfo {
            hash: hasher.finish().to_string(),
        }
    }

    fn get_hash(&self) -> &str {
        self.hash.as_str()
    }
}
