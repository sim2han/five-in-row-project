use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub pwd: String,
    pub rating: u32,
    pub code: String,
}


impl UserInfo {
    pub fn new(id: String, pwd: String, rating: u32) -> Self {
        let code = id.clone();
        UserInfo {
            id,
            pwd,
            rating: 600,
            code,
        }
    }
}

