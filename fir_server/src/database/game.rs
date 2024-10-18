use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeControl {
    pub seconds: u32,
    pub fisher: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Into<fir_game::Coord> for Coord {
    fn into(self) -> fir_game::Coord {
        fir_game::Coord {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Command {
    Play(Side, Coord),
    Resign,
    OfferDraw,
    AcceptDraw,
}
