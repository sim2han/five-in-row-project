use crate::{
    database::data::{self, TimeControl},
    game_queue::GameInitData,
    prelude::*,
};

use hyper::upgrade::Upgraded;
use hyper_tungstenite::{HyperWebsocket, WebSocketStream};
use hyper_util::rt::TokioIo;
use std::collections::VecDeque;
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
pub struct UserRegisterData {
    pub data: data::UserData,
    stream: Option<HyperWebsocket>,
    pub open_stream: Option<WebSocketStream<TokioIo<Upgraded>>>,
}

impl UserRegisterData {
    pub fn new(data: data::UserData, stream: HyperWebsocket) -> Self {
        Self {
            data,
            stream: Some(stream),
            open_stream: None,
        }
    }

    // connect stream
    async fn connect(&mut self) {
        let stream = self.stream.take().unwrap();
        self.open_stream = Some(stream.await.unwrap());
    }

    fn get_stream(&mut self) -> &mut WebSocketStream<TokioIo<Upgraded>> {
        self.open_stream.as_mut().unwrap()
    }
}

/// Match queue
///
/// Basically, match queue matches two client into a game.
pub struct MatchQueue {
    queue: VecDeque<UserRegisterData>,
    sender: Sender<UserRegisterData>,
    receiver: Receiver<UserRegisterData>,
}

impl MatchQueue {
    /// Create a new match queue
    pub fn new() -> Self {
        let (sender, receiver) = channel(100);
        Self {
            queue: VecDeque::new(),
            sender: sender,
            receiver: receiver,
        }
    }

    pub fn get_sender(&mut self) -> Sender<UserRegisterData> {
        Sender::clone(&self.sender)
    }

    pub async fn run(mut self, gameq: Sender<GameInitData>) {
        log("match queue start!");

        loop {
            let mut userdata = self.receiver.recv().await.unwrap();
            // wait until websocket connection finished.
            userdata.connect().await;
            log("connect complete");

            self.queue.push_back(userdata);
            log(&format!("current queue size: {:?}", self.queue.len()));

            // make match
            if self.queue.len() >= 2 {
                let player1 = self.queue.pop_front().unwrap();
                let player2 = self.queue.pop_front().unwrap();

                let init = GameInitData::new(
                    player1,
                    player2,
                    TimeControl {
                        seconds: 100,
                        fisher: 0,
                    },
                );

                super::utility::log("make match");
                gameq.send(init).await.unwrap();
            }
        }
    }
}
