use std::thread;
use std::sync::mpsc::{self, Receiver};
use super::game_queue;
use std::collections::VecDeque;

/// user information for matching
struct UserInfo {
    ip_address: String,
    user_name: String,
    rating: i32,
}

struct MatchQueue {
    queue: VecDeque<UserInfo>,
}

impl MatchQueue {
    /// Create a new match queue
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

pub fn start_match_queue_thread() -> mpsc::Sender<UserInfo> {
    let (sender, receiver) = mpsc::channel::<UserInfo>();
    thread::spawn(move || {
        loop {
            match receiver.recv() {
                Ok(userInfo) => {

                }
                Err(e) => {
                    println!("Error : {e:?}");
                }
            }
        }
    });
    sender
}
