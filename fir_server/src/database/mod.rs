use std::sync::Arc;
use crate::prelude::*;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

mod user_info;

pub use user_info::UserInfo;

/**
 * Save game play data
 */

pub struct RealData {
    user_infos: Vec<user_info::UserInfo>,
    game_results: Vec<GameResult>,
}

impl RealData {
    fn new() -> Self {
        RealData {
            user_infos: vec![],
            game_results: vec![],
        }
    }

    fn add_user_data(&mut self, data: user_info::UserInfo) {
        log(&format!("add user data {:?}", data));
        self.user_infos.push(data);
    }

    fn add_game_result(&mut self, data: GameResult) {
        log(&format!("add game result {:?}", data));
        self.game_results.push(data);
    }

    pub fn get_user_data(&mut self, name: String) -> Option<&user_info::UserInfo> {
        for info in self.user_infos.iter() {
            if info.name == name {
                return Some(info);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct GameResult {}

pub enum UpdateQuery {
    UserInfo(user_info::UserInfo),
    GameResult(GameResult),
}

pub struct Database {
    data: Arc<Mutex<RealData>>,
    sender: Sender<UpdateQuery>,
    receiver: Receiver<UpdateQuery>,
}

pub type DbSender = Sender<UpdateQuery>;

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

    pub fn get_db(&self) -> Arc<Mutex<RealData>> {
        Arc::clone(&self.data)
    }

    pub async fn run(mut self) {
        log("db start!");

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
