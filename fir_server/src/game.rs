/**
 * Game player
 */
use crate::database::data::Coord;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub enum GameCommand {
    Play(Coord),
    Resign,
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("unkown command recieved: {0}")]
    UnkownCommand(String),
}

fn parse_command(command: String) -> Result<GameCommand, GameError> {
    let mut command = command.split_whitespace();

    let first = command.next().unwrap();

    match first {
        "play" => {
            let x = command.next().unwrap().parse::<u32>().unwrap();
            let y = command.next().unwrap().parse::<u32>().unwrap();
            Ok(GameCommand::Play(Coord { x, y }))
        }
        "resign" => Ok(GameCommand::Resign),
        _ => Err(GameError::UnkownCommand(String::from(first))),
    }
}
