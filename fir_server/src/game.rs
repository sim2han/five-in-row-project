/**
 * Game player
 */
use fir_game;
use thiserror::Error;

#[derive(Debug)]
pub enum GameCommand {
    Play(fir_game::Coord),
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
            Ok(GameCommand::Play(fir_game::Coord { x, y }))
        }
        "resign" => Ok(GameCommand::Resign),
        _ => Err(GameError::UnkownCommand(String::from(first))),
    }
}
