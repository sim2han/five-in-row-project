use crate::{database::data::TimeControl, game_queue::GameInitData, prelude::*};

use futures::sink::SinkExt;
use futures::stream::StreamExt;
use hyper::upgrade::Upgraded;
use hyper_tungstenite::{tungstenite, HyperWebsocket, WebSocketStream};
use hyper_util::rt::TokioIo;
use std::collections::VecDeque;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tungstenite::Message;

#[derive(Debug)]
pub struct UserRegisterData {
    //stream: hyper_tungstenite::WebSocketStream<TokioIo<Upgraded>>,
    stream: Option<HyperWebsocket>,
    open_stream: Option<WebSocketStream<TokioIo<Upgraded>>>,
}

impl UserRegisterData {
    //pub fn new(stream: hyper_tungstenite::WebSocketStream<TokioIo<Upgraded>>) -> Self {
    pub fn new(stream: HyperWebsocket) -> Self {
        Self {
            stream: Some(stream),
            open_stream: None,
        }
    }

    // connect stream
    async fn connect(&mut self) {
        let stream = self.stream.take().unwrap();
        self.open_stream = Some(stream.await.unwrap());
    }

    fn get_stream(&mut self) -> &WebSocketStream<TokioIo<Upgraded>> {
        &self.open_stream.as_ref().unwrap()
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
            let mut resv = self.receiver.recv().await.unwrap();
            resv.connect().await;
            log("connect complete");

            resv.open_stream
                .as_mut()
                .unwrap()
                .send(Message::text("tk"))
                .await
                .unwrap();

            resv.open_stream
                .as_mut()
                .unwrap()
                .send(Message::text("good"))
                .await
                .unwrap();

            loop {
                let m = resv.open_stream.as_mut().unwrap().next().await;
                let websocket = resv.open_stream.as_mut().unwrap();

                if let Some(ref message) = m {
                    match message {
                        Ok(message) => {
                            match message {
                                Message::Text(msg) => {
                                    println!("Received text message: {msg}");
                                    websocket
                                        .send(Message::text("Thank you, come again."))
                                        .await
                                        .unwrap();
                                }
                                Message::Binary(msg) => {
                                    println!("Received binary message: {msg:02X?}");
                                    websocket
                                        .send(Message::binary(b"Thank you, come again.".to_vec()))
                                        .await
                                        .unwrap();
                                }
                                Message::Ping(msg) => {
                                    // No need to send a reply: tungstenite takes care of this for you.
                                    println!("Received ping message: {msg:02X?}");
                                }
                                Message::Pong(msg) => {
                                    println!("Received pong message: {msg:02X?}");
                                }
                                Message::Close(msg) => {
                                    // No need to send a reply: tungstenite takes care of this for you.
                                    if let Some(msg) = &msg {
                                        println!(
                                            "Received close message with code {} and message: {}",
                                            msg.code, msg.reason
                                        );
                                    } else {
                                        println!("Received close message");
                                    }
                                    break;
                                }
                                Message::Frame(_msg) => {
                                    unreachable!();
                                }
                            }
                        }
                        Err(e) => {
                            log(format!("Error: {e}").as_str());
                        }
                    }
                } else {
                    break;
                }
            }

            self.queue.push_back(resv);
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
            }
        }
    }
}
/*
pub fn start_match_queue_thread() -> mpsc::Sender<UserInfo> {
    let (sender, receiver) = mpsc::channel::<UserInfo>();
    thread::spawn(move || loop {
        match receiver.recv() {
            Ok(userInfo) => {}
            Err(e) => {
                println!("Error : {e:?}");
            }
        }
    });
    sender
}*/
