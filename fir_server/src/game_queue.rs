/**
 * Game pool
 */
use super::thread_pool;
use fir_game;

use hyper::upgrade::Upgraded;
use hyper_tungstenite::WebSocketStream;
use hyper_util::rt::TokioIo;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct GameInitData {
    player1: WebSocketStream<TokioIo<Upgraded>>,
    player2: WebSocketStream<TokioIo<Upgraded>>,
}

impl GameInitData {
    pub fn new(
        player1: WebSocketStream<TokioIo<Upgraded>>,
        player2: WebSocketStream<TokioIo<Upgraded>>,
    ) -> Self {
        GameInitData { player1, player2 }
    }
}

pub struct GameQueue {
    sender: Sender<GameInitData>,
    receiver: Receiver<GameInitData>,
}

impl GameQueue {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        GameQueue { sender, receiver }
    }

    pub fn get_sender(&self) -> Sender<GameInitData> {
        self.sender.clone()
    }

    pub async fn run(mut self) {
        loop {
            let recv = self.receiver.recv().await.unwrap();

            let gameroom = GameRoom::from_data(recv);
            tokio::spawn(gameroom.run());
        }
    }
}

pub struct GameRoom {
    // 0: black, 1: white
    streams: [hyper_tungstenite::WebSocketStream<TokioIo<Upgraded>>; 2],
    game: fir_game::FirGame,
}

impl GameRoom {
    pub fn new(
        player1: WebSocketStream<TokioIo<Upgraded>>,
        player2: WebSocketStream<TokioIo<Upgraded>>,
    ) -> Self {
        GameRoom {
            streams: [player1, player2],
            game: fir_game::FirGame::new(),
        }
    }

    pub fn from_data(data: GameInitData) -> Self {
        GameRoom {
            streams: [data.player1, data.player2],
            game: fir_game::FirGame::new(),
        }
    }

    // this function bring its data,
    // so data will be deleted when this function ends
    pub async fn run(self) {}
}
