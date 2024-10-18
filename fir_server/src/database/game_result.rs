use super::{
    game::{Side, TimeControl},
    user_info::UserInfo,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GameResult {
    Win(Side),
    Resign(Side),
    Draw,
    Abort,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub black_name: UserInfo,
    pub white_name: UserInfo,
    pub result: GameResult,
    pub time: TimeControl,
}
