use crate::database::data;
/**
 * Game player
 */
use crate::database::data::Notation;
use thiserror::Error;
/*
#[derive(Error, Debug)]
pub enum GameError {
    #[error("unkown command recieved: {0}")]
    UnkownCommand(String),
}

fn parse_command(command: String) -> Result<data::GameCommand, GameError> {
    let mut command = command.split_whitespace();

    let first = command.next().unwrap();

    match first {
        "play" => {
            let x = command.next().unwrap().parse::<u32>().unwrap();
            let y = command.next().unwrap().parse::<u32>().unwrap();
            Ok(data::GameCommand::Play(data::Notation { x, y }))
        }
        "resign" => Ok(data::GameCommand::Resign),
        _ => Err(GameError::UnkownCommand(String::from(first))),
    }
}
*/
