use fir_game;

use crate::{database::data::*, match_queue::UserRegisterData};
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
    pub async fn run(self, sender: Sender<crate::database::UpdateQuery>) {
        let (s, mut r) = channel(10);

        let ms = s.clone();
        tokio::spawn(Self::player1_receive(ms.clone()));

        let ms = s.clone();
        tokio::spawn(Self::player2_receive(ms.clone()));

        loop {
            let message = r.recv().await.unwrap();
        }
    }

    async fn player1_receive(sender: Sender<PlayCommand>) {}

    async fn player2_receive(sender: Sender<PlayCommand>) {}
}

/// command interthrowd by client and server
#[derive(Debug, Clone, Copy)]
struct PlayCommand {
    side: Side,
}
