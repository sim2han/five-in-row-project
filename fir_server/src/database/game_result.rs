use super::{
    game::{Coord, Side, TimeControl},
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
    pub black_user: UserInfo,
    pub white_user: UserInfo,
    pub result: GameResult,
    pub time: TimeControl,
    pub notation: Vec<Coord>,
}
