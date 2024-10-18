mod game;
mod game_result;
mod user_info;

use crate::prelude::*;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

pub mod data {
    pub use super::game::{Coord, Side, TimeControl};
    pub use super::game_result::{GameInfo, GameResult};
    pub use super::user_info::UserInfo;
}
/**
 * Save game play data
 */

pub struct RealData {
    user_infos: Vec<user_info::UserInfo>,
    game_results: Vec<data::GameResult>,
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

    fn add_game_result(&mut self, data: data::GameResult) {
        log(&format!("add game result {:?}", data));
        self.game_results.push(data);
    }

    pub fn get_user_data(&mut self, id: String) -> Option<&user_info::UserInfo> {
        for info in self.user_infos.iter() {
            if info.id == id {
                return Some(info);
            }
        }
        None
    }

    pub fn get_all_user(&self) -> &Vec<data::UserInfo> {
        &self.user_infos
    }

    pub fn get_all_user_serizlie(
        &self,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let str = serde_json::to_string(&self.user_infos)?;
        Ok(str)
    }
}

#[derive(Debug)]
pub enum UpdateQuery {
    UserInfo(user_info::UserInfo),
    GameResult(data::GameResult),
}

pub struct DataManager {
    data: Arc<Mutex<RealData>>,
    sender: Sender<UpdateQuery>,
    receiver: Receiver<UpdateQuery>,
}

pub type DbSender = Sender<UpdateQuery>;

impl DataManager {
    pub fn new() -> Self {
        let (s, r) = tokio::sync::mpsc::channel(10);

        DataManager {
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

        while let Some(value) = self.receiver.recv().await {
            let mut data = self.data.lock().await;

            match value {
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
