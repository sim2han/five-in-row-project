/**
 * Game pool
 */
use super::thread_pool;
use fir_game;

use hyper::upgrade::Upgraded;
use hyper_tungstenite::WebSocketStream;
use hyper_util::rt::TokioIo;
use tokio::sync::mpsc::{channel, Receiver, Sender};

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
    sender: tokio::sync::mpsc::Sender<GameInitData>,
    receiver: tokio::sync::mpsc::Receiver<GameInitData>,
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
    pub async fn run(self, sender: Sender<crate::database::UpdateQuery>) {
        let (s, mut r) = channel(10);

        let ms = s.clone();
        tokio::spawn(async move {
            loop {
                GameRoom::player1_receive(ms.clone()).await;
            }
        });

        let ms = s.clone();
        tokio::spawn(async move {
            loop {
                GameRoom::player2_receive(ms.clone()).await;
            }
        });

        loop {
            let message = r.recv().await.unwrap();
        }
    }

    async fn player1_receive(sender: Sender<PlayCommand>) {}

    async fn player2_receive(sender: Sender<PlayCommand>) {}
}

#[derive(Debug, Clone, Copy)]
enum Side {
    Black,
    White,
    None,
}

/// command interthrowd by client and server
#[derive(Debug, Clone, Copy)]
struct PlayCommand {
    side: Side,
}
