use std::result;

use fir_game;

use crate::database::{data, info};
use crate::socket::Socket;
use crate::{database::data::*, match_queue::UserRegisterData, prelude::*};
use hyper::upgrade::Upgraded;
use hyper_tungstenite::WebSocketStream;
use hyper_util::rt::TokioIo;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct GameInitData {
    player1: UserRegisterData,
    player2: UserRegisterData,
    time: TimeControl,
}

impl GameInitData {
    pub fn new(player1: UserRegisterData, player2: UserRegisterData, time: TimeControl) -> Self {
        GameInitData {
            player1,
            player2,
            time,
        }
    }
}

///
/// <examples>
///
pub struct GameQueue {
    sender: Sender<GameInitData>,
    receiver: Receiver<GameInitData>,
}

impl GameQueue {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        GameQueue {
            sender: tx,
            receiver: rx,
        }
    }

    pub fn get_sender(&self) -> Sender<GameInitData> {
        self.sender.clone()
    }

    pub async fn run(mut self, sender: Sender<crate::database::UpdateQuery>) {
        loop {
            let recv = self.receiver.recv().await.unwrap();

            let gameroom = GameRoom::from_data(recv);
            tokio::spawn(gameroom.run(sender.clone()));
        }
    }
}

pub struct GameRoom {
    // 0: black, 1: white
    users: [UserRegisterData; 2],
    game: fir_game::FirGame,
}

impl GameRoom {
    pub fn new(player1: UserRegisterData, player2: UserRegisterData) -> Self {
        GameRoom {
            users: [player1, player2],
            game: fir_game::FirGame::new(),
        }
    }

    pub fn from_data(data: GameInitData) -> Self {
        GameRoom {
            users: [data.player1, data.player2],
            game: fir_game::FirGame::new(),
        }
    }

    // this function bring its data,
    // so data will be deleted when this function ends
    pub async fn run(mut self, sender: Sender<crate::database::UpdateQuery>) {
        log("Game Start!");

        // make socket handler
        let socket0 = Socket::new(self.users[0].open_stream.take().unwrap());
        let socket1 = Socket::new(self.users[1].open_stream.take().unwrap());

        let (player0_tx, mut player0_rx) = socket0.get_channel();
        let (player1_tx, mut player1_rx) = socket1.get_channel();

        tokio::spawn(socket0.run());
        tokio::spawn(socket1.run());

        // make two receiver to one receiver with mpsc
        let (tx, mut rx) = channel(10);

        let tx1 = Sender::clone(&tx);
        tokio::spawn(async move {
            loop {
                if let Ok(message) = player0_rx.recv().await {
                    if let Stopper::Go(message) = message {
                        tx1.send(message).await.unwrap();
                    } else {
                        break;
                    }
                }
            }
        });

        let tx2 = Sender::clone(&tx);
        tokio::spawn(async move {
            loop {
                if let Ok(message) = player1_rx.recv().await {
                    if let Stopper::Go(message) = message {
                        tx2.send(message).await.unwrap();
                    } else {
                        break;
                    }
                }
            }
        });

        // send color
        let command = data::GameResponse::Start(Side::Black);
        let command: info::GameResponseInfo = command.into();
        let command = serde_json::to_string(&command).unwrap();
        player0_tx.send(Stopper::Go(command)).unwrap();

        let command = data::GameResponse::Start(Side::White);
        let command: info::GameResponseInfo = command.into();
        let command = serde_json::to_string(&command).unwrap();
        player1_tx.send(Stopper::Go(command)).unwrap();

        // make game
        let mut game = fir_game::FirGame::new();
        let mut gamedata = data::GameData::new(self.users[0].data.clone(), self.users[1].data.clone());
        //gamedata.black_user = self.users[0].data.clone();
        //gamedata.white_user = self.users[1].data.clone();

        // game command handler
        tokio::spawn(async move {
            loop {
                if let Some(message) = rx.recv().await {
                    log(&format!("game receive message: {message:?}"));
                    let command: info::GameCommandInfo = serde_json::from_str(&message).unwrap();
                    let command: data::GameCommand = command.into();

                    match command.command_type {
                        data::CommandType::Message => {
                            let response = data::GameResponse::Message(command.message);
                            let response: info::GameResponseInfo = response.into();
                            let response = serde_json::to_string(&response).unwrap();
                            if let Side::Black = command.side {
                                player1_tx.send(Stopper::Go(response)).unwrap();
                            } else {
                                player0_tx.send(Stopper::Go(response)).unwrap();
                            }
                        }
                        data::CommandType::Play => {
                            gamedata.notations.push(command.notation);
                            game.play(command.notation.x, command.notation.y).unwrap();
                            log(&game.board_state());
                            let response = data::GameResponse::OpponentPlay(command.notation);
                            let response: info::GameResponseInfo = response.into();
                            let response = serde_json::to_string(&response).unwrap();
                            if let Side::Black = command.side {
                                player1_tx.send(Stopper::Go(response)).unwrap();
                            } else {
                                player0_tx.send(Stopper::Go(response)).unwrap();
                            }
                        }
                        data::CommandType::Resign => {
                            let response = data::GameResponse::OpponentResign;
                            let response: info::GameResponseInfo = response.into();
                            let response = serde_json::to_string(&response).unwrap();
                            if let Side::Black = command.side {
                                player1_tx.send(Stopper::Go(response)).unwrap();
                            } else {
                                player0_tx.send(Stopper::Go(response)).unwrap();
                            }

                            // send game end response
                            let response = data::GameResponse::GameEnd;
                            let response: info::GameResponseInfo = response.into();
                            let response = serde_json::to_string(&response).unwrap();
                            player1_tx.send(Stopper::Go(response.clone())).unwrap();
                            player0_tx.send(Stopper::Go(response)).unwrap();

                            // stop async functions
                            player1_tx.send(Stopper::Stop).unwrap();
                            player0_tx.send(Stopper::Stop).unwrap();

                            gamedata.result = GameResult::Resign(command.side);
                            sender
                                .send(crate::database::UpdateQuery::GameData(gamedata))
                                .await
                                .unwrap();
                            break;
                        }
                        _ => (),
                    }
                }
            }
        })
        .await
        .unwrap();

        log("Game End!");
    }
}
