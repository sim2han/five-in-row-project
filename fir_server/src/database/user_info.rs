use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub pwd: String,
    pub rating: u32,
}

/*
impl UserInfo {
    pub fn from_info(id: String, pwd: String, rating: u32) -> Self {
        UserInfo {
            id,
            pwd,
            rating: 600,
        }
    }
}
*/
