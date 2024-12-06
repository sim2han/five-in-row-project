use crate::prelude::*;
use crate::utility::Stopper;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use hyper::upgrade::Upgraded;
use hyper_tungstenite::WebSocketStream;
use hyper_util::rt::TokioIo;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tungstenite::Message;

/// WebSocket wrapper
pub struct Socket {
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
        let h1 = tokio::spawn(async move {
            let tx = self.tx_out.clone();
            loop {
                let message = self.read.next().await;
                if let Some(ref message) = message {
                    match message {
                        Ok(message) => match message {
                            Message::Text(msg) => {
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

        let h2 = tokio::spawn(async move {
            while let Ok(message) = self.rx_in.recv().await {
                if let Stopper::Go(ref message) = message {
                    if true {
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
