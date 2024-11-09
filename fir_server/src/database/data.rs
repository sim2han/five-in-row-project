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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Notation {
    pub color: bool,
    pub x: u32,
    pub y: u32,
}

impl Into<info::NotationInfo> for Notation {
    fn into(self) -> info::NotationInfo {
        NotationInfo {
            color: if self.color { 1 } else { 0 },
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<Notation> for info::NotationInfo {
    fn into(self) -> Notation {
        Notation {
            color: if self.color == 1 { true } else { false },
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
    OnGoing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub result: GameResult,
    pub black_user: UserData,
    pub white_user: UserData,
    //pub time: TimeControl,
    pub notations: Vec<Notation>,
}

impl GameData {
    pub fn new(black_user: UserData, white_user: UserData) -> Self {
        GameData {
            result: GameResult::OnGoing,
            black_user,
            white_user,
            notations: vec![],
        }
    }
}

impl Into<info::GameInfo> for GameData {
    fn into(self) -> info::GameInfo {
        let result = match self.result {
            GameResult::Win(Side::White) => "white win",
            GameResult::Win(Side::Black) => "black win",
            GameResult::Resign(Side::White) => "white resign",
            GameResult::Resign(Side::Black) => "black resign",
            GameResult::Draw => "draw",
            GameResult::Abort => "abort",
            _ => unreachable!(),
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
pub struct GameCommand {
    pub side: Side,
    pub command_type: CommandType,
    pub notation: Notation,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum CommandType {
    Play,
    Resign,
    OfferDraw,
    AcceptDraw,
    Message,
}

impl Into<GameCommand> for info::GameCommandInfo {
    fn into(self) -> GameCommand {
        let resp = self.command;
        let command_type = if resp == "Play" {
            CommandType::Play
        } else if resp == "Resign" {
            CommandType::Resign
        } else if resp == "OfferDraw" {
            CommandType::OfferDraw
        } else if resp == "AcceptDraw" {
            CommandType::AcceptDraw
        } else if resp == "Message" {
            CommandType::Message
        } else {
            unreachable!("Unkown Command")
        };

        let side = if self.side == 0 {
            Side::Black
        } else {
            Side::White
        };

        GameCommand {
            command_type,
            message: self.message,
            notation: self.notation.into(),
            side,
        }
    }
}

#[derive(Debug, Clone)]
pub enum GameResponse {
    Start(Side),
    OpponentPlay(Notation),
    OpponentResign,
    OpponentOfferDraw,
    GameEnd,
    Message(String),
}

impl Into<info::GameResponseInfo> for GameResponse {
    fn into(self) -> info::GameResponseInfo {
        let command = match self {
            GameResponse::Start(_) => "Start",
            GameResponse::OpponentPlay(_) => "OpponentPlay",
            GameResponse::OpponentResign => "OpponentResign",
            GameResponse::OpponentOfferDraw => "OpponentOfferDraw",
            GameResponse::GameEnd => "GameEnd",
            GameResponse::Message(_) => "Message",
        };
        let notation = match self {
            GameResponse::Start(Side::Black) => NotationInfo {
                color: 0,
                ..NotationInfo::default()
            },
            GameResponse::Start(Side::White) => NotationInfo {
                color: 1,
                ..NotationInfo::default()
            },
            GameResponse::OpponentPlay(n) => n.into(),
            _ => NotationInfo::default(),
        };
        let message = match self {
            GameResponse::Message(s) => s,
            _ => String::new(),
        };
        info::GameResponseInfo {
            command: String::from(command),
            notation,
            message,
        }
    }
}
