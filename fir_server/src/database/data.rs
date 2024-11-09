/// 게임 전반으로 사용되는 데이터들
use super::info::{self, NotationInfo};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserData {
    pub id: String,
    pub pwd: String,
    pub rating: u32,
    pub key: String,
}

impl Into<info::UserInfo> for UserData {
    fn into(self) -> info::UserInfo {
        info::UserInfo {
            id: self.id,
            pwd: self.pwd,
            rating: self.rating,
            key: self.key,
        }
    }
}

impl Into<UserData> for info::UserInfo {
    fn into(self) -> UserData {
        UserData {
            id: self.id,
            pwd: self.pwd,
            rating: self.rating,
            key: self.key,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TimeControl {
    pub seconds: u32,
    pub fisher: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notation {
    pub is_black: bool,
    pub x: u32,
    pub y: u32,
}

impl Into<info::NotationInfo> for Notation {
    fn into(self) -> info::NotationInfo {
        NotationInfo {
            isblack: if self.is_black { 1 } else { 0 },
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<Notation> for info::NotationInfo {
    fn into(self) -> Notation {
        Notation {
            is_black: if self.isblack == 1 { true } else { false },
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<fir_game::Coord> for Notation {
    fn into(self) -> fir_game::Coord {
        fir_game::Coord {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameResult {
    Win(Side),
    Resign(Side),
    Draw,
    Abort,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub result: GameResult,
    pub black_user: UserData,
    pub white_user: UserData,
    //pub time: TimeControl,
    pub notations: Vec<Notation>,
}

impl Into<info::GameInfo> for GameData {
    fn into(self) -> info::GameInfo {
        let result = match self.result {
            GameResult::Win(Side::White) => "white win",
            GameResult::Win(Side::Black) => "black win",
            GameResult::Resign(Side::White) => "white resign",
            GameResult::Resign(Side::Black) => "blakc resign",
            GameResult::Draw => "draw",
            GameResult::Abort => "abort",
        };
        info::GameInfo {
            result: String::from(result),
            blackname: self.black_user.id.clone(),
            blackrating: self.black_user.rating,
            whitename: self.white_user.id.clone(),
            whiterating: self.white_user.rating,
            notations: self.notations.into_iter().map(|n| n.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, Clone)]
pub enum GameCommand {
    Play(Notation),
    Resign,
    OfferDraw,
    AcceptDraw,
}

impl Into<GameCommand> for info::GameCommandInfo {
    fn into(self) -> GameCommand {
        let resp = self.command;
        let notation = self.notation;

        if resp == "Play" {
            GameCommand::Play(notation.into())
        } else if resp == "Resign" {
            GameCommand::Resign
        } else if resp == "OfferDraw" {
            GameCommand::OfferDraw
        } else if resp == "AcceptDraw" {
            GameCommand::AcceptDraw
        } else {
            unreachable!("Unkown Command")
        }
    }
}
