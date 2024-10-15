use std::sync::Arc;

use tokio::sync::{mpsc::{Receiver, Sender}, Mutex};

/**
 * Save game play data
 */

struct RealData {
    user_infos: Vec<super::user_info::UserInfo>,
    game_results: Vec<GameResult>,
}

impl RealData {
    fn new() -> Self {
        RealData {
            user_infos: vec![],
            game_results: vec![],
        }
    }

    fn add_user_data(&mut self, data: super::user_info::UserInfo) {
        self.user_infos.push(data);
    }

    fn add_game_result(&mut self, data: GameResult) {
        self.game_results.push(data);
    }

    fn get_user_data(&mut self, name: String) -> Option<&super::user_info::UserInfo> {
        for info in self.user_infos.iter() {
            if info.id == name {
                return Some(info);
            }
        }
        None
    }
}

pub struct GameResult {
    
}

pub enum UpdateQuery {
    UserInfo(super::user_info::UserInfo),
    GameResult(GameResult),
}

pub struct Database {
    data: Arc<Mutex<RealData>>,
    sender: Sender<UpdateQuery>,
    receiver: Receiver<UpdateQuery>,
}

impl Database {
    pub fn new() -> Self {
        let (s, r) = tokio::sync::mpsc::channel(100);

        Database {
            data: Arc::new(Mutex::new(RealData::new())),
            sender: s,
            receiver: r,
        }
    }

    pub fn get_sender(&self) -> Sender<UpdateQuery> {
        Sender::clone(&self.sender)
    }

    pub async fn run(mut self) {
        
        loop {
            let recv = self.receiver.recv().await.unwrap();
            let mut data = self.data.lock().await;

            match recv {
                UpdateQuery::GameResult(s) => {
                    data.add_game_result(s);
                }
                UpdateQuery::UserInfo(s) => {
                    data.add_user_data(s);
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
}
