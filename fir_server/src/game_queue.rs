use fir_game;

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
        log("Game Room Start");

        // make socket handler
        let socket0 = Socket::new(self.users[0].open_stream.take().unwrap());
        let socket1 = Socket::new(self.users[1].open_stream.take().unwrap());

        let (player0_tx, mut player0_rx) = socket0.get_channel();
        let (player1_tx, mut player1_rx) = socket1.get_channel();

        tokio::spawn(socket0.run());
        tokio::spawn(socket1.run());

        let (tx, mut rx) = channel(10);

        let tx1 = Sender::clone(&tx);
        tokio::spawn(async move {
            loop {
                if let Ok(message) = player0_rx.recv().await {
                    if let Stopper::Go(message) = message {
                        log(&format!("Seeeeend {}", message));
                        tx1.send(message).await;
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
                        tx2.send(message).await;
                    } else {
                        break;
                    }
                } else {
                    //log("bbbbb");
                }
            }
        });

        player0_tx.send(Stopper::Go(String::from("asdfasdf")));

        let handle = tokio::spawn(async move {
            loop {
                while let Some(message) = rx.recv().await {
                    log(&format!("game receive message: {message:?}"));

                    player0_tx.send(Stopper::Go(message.clone()));
                    player1_tx.send(Stopper::Go(message));
                }
            }
        });

        handle.await;
    }
}

/// command interthrowd by client and server
#[derive(Debug, Clone, Copy)]
struct PlayCommand {
    side: Side,
}
