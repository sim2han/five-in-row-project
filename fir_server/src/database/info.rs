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
pub struct IdPwdInfo {
    pub id: String,
    pub pwd: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub pwd: String,
    pub rating: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotationInfo {
    pub is_black: bool,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub result: String,
    pub black: UserInfo,
    pub white: UserInfo,
    pub notations: Vec<NotationInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameCommandInfo {
    pub command: String,
    pub notation: NotationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResponseInfo {
    pub response: String,
    pub notation: NotationInfo,
}
