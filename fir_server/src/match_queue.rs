use super::game_queue;
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;
use tokio::sync::mpsc::{self, Receiver, Sender};

/// user information for matching
pub struct UserInfo {
    ip_address: String,
    user_name: String,
    rating: i32,
}

#[derive(Debug)]
pub struct UserRegisterData {
    stream: tokio::net::TcpStream,
}

impl UserRegisterData {
    pub fn new(stream: tokio::net::TcpStream) -> Self {
        Self { stream }
    }
}

/// Match queue
/// Basically, match queue matches two client into a game.
pub struct MatchQueue {
    queue: VecDeque<UserRegisterData>,
    sender: Sender<UserRegisterData>,
    receiver: Receiver<UserRegisterData>,
}

impl MatchQueue {
    /// Create a new match queue
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(100);
        Self {
            queue: VecDeque::new(),
            sender: sender,
            receiver: receiver,
        }
    }

    pub fn get_sender(&mut self) -> Sender<UserRegisterData> {
        Sender::clone(&self.sender)
    }

    pub async fn run(mut self) {
        loop {
            let resv = self.receiver.recv().await.unwrap();

            println!("push {:?}", resv);
            self.queue.push_back(resv);
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
