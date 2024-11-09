use crate::prelude::*;
use crate::utility::Stopper;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use hyper::upgrade::Upgraded;
use hyper_tungstenite::WebSocketStream;
use hyper_util::rt::TokioIo;
use std::sync::Arc;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tungstenite::Message;

/// WebSocket wrapper
pub struct Socket {
    //websocket: Arc<Mutex<WebSocketStream<TokioIo<Upgraded>>>>,
    tx_in: Sender<Stopper<String>>,
    rx_in: Receiver<Stopper<String>>,
    tx_out: Sender<Stopper<String>>,
    write: SplitSink<WebSocketStream<TokioIo<Upgraded>>, Message>,
    read: SplitStream<WebSocketStream<TokioIo<Upgraded>>>,
}

impl Socket {
    pub fn new(websocket: WebSocketStream<TokioIo<Upgraded>>) -> Self {
        let (tx_out, _) = channel(10);
        let (tx_in, rx_in) = channel(10);
        let (write, read) = websocket.split();
        Socket {
            //websocket: Arc::new(Mutex::new(websocket)),
            tx_in,
            rx_in,
            tx_out,
            write,
            read,
        }
    }

    pub fn get_channel(&self) -> (Sender<Stopper<String>>, Receiver<Stopper<String>>) {
        (Sender::clone(&self.tx_in), self.tx_out.subscribe())
    }

    pub async fn run(mut self) {
        // websocket receiver
        //let socketc = self.websocket.clone();
        let h1 = tokio::spawn(async move {
            let tx = self.tx_out.clone();
            loop {
                //let mut socket = socketc.lock().await;
                //let message = socket.next();
                let message = self.read.next().await;
                if let Some(ref message) = message {
                    match message {
                        Ok(message) => match message {
                            Message::Text(msg) => {
                                //log(&format!("Received text message: {msg:?}"));
                                tx.send(Stopper::Go(String::from(msg))).unwrap();
                            }
                            _ => (),
                        },
                        Err(e) => {
                            log(format!("Error: {e}").as_str());
                        }
                    }
                } else {
                    break;
                }
            }

            self.tx_out.send(Stopper::Stop).unwrap();
        });

        // websocket sender
        //let socketc = self.websocket.clone();
        let h2 = tokio::spawn(async move {
            while let Ok(message) = self.rx_in.recv().await {
                if let Stopper::Go(ref message) = message {
                    if true {
                        //let mut socket = socketc.lock().await;
                        //log(&format!("send text message {message}"));
                        //socket.send(Message::text(message)).await.unwrap();
                        self.write.send(Message::text(message)).await.unwrap();
                    }
                } else {
                    break;
                }
            }
        });

        let _ = tokio::join!(h1, h2);
    }
}
