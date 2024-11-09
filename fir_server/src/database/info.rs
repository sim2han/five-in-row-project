// 클라이언트와 주고 받는 것 serialzable 데이터
// rust enum처리가 미묘해서 다 string으로

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterInfo {
    pub id: String,
    pub pwd: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserKeyInfo {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginInfo {
    pub id: String,
    pub pwd: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub pwd: String,
    pub rating: u32,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotationInfo {
    pub color: u32,
    pub x: u32,
    pub y: u32,
}

impl Default for NotationInfo {
    fn default() -> Self {
        NotationInfo {
            color: 2,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub result: String,
    pub blackname: String,
    pub blackrating: u32,
    pub whitename: String,
    pub whiterating: u32,
    pub notations: Vec<NotationInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameCommandInfo {
    pub side: u32,
    pub command: String,
    pub notation: NotationInfo,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResponseInfo {
    pub command: String,
    pub notation: NotationInfo,
    pub message: String,
}
