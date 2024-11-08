pub mod data;
pub mod info;

use crate::prelude::*;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

pub mod load {
    pub use super::data;
    pub use super::info;
}
/**
 * Save game play data
 */

pub struct Database {
    user_infos: Vec<data::UserData>,
    game_results: Vec<data::GameData>,
}

impl Database {
    fn new() -> Self {
        Database {
            user_infos: vec![],
            game_results: vec![],
        }
    }

    fn add_user_data(&mut self, data: data::UserData) {
        log(&format!("add user data {:?}", data));
        self.user_infos.push(data);
    }

    fn add_game_result(&mut self, data: data::GameData) {
        log(&format!("add game result {:?}", data));
        self.game_results.push(data);
    }

    pub fn get_user_data(&mut self, id: String) -> Option<&data::UserData> {
        for info in self.user_infos.iter() {
            if info.id == id {
                return Some(info);
            }
        }
        None
    }

    pub fn get_all_user(&self) -> &Vec<data::UserData> {
        &self.user_infos
    }



    pub fn get_all_game(&self) -> &Vec<data::GameData> {
        &self.game_results
    }

    // 테스트용
    pub fn get_all_game_serialize(
        &self,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let str = serde_json::to_string(&self.game_results)?;
        Ok(str)
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
    UserData(data::UserData),
    GameData(data::GameData),
}

pub struct DataManager {
    data: Arc<Mutex<Database>>,
    sender: Sender<UpdateQuery>,
    receiver: Receiver<UpdateQuery>,
}

pub type DbSender = Sender<UpdateQuery>;

impl DataManager {
    pub fn new() -> Self {
        let (s, r) = tokio::sync::mpsc::channel(10);

        DataManager {
            data: Arc::new(Mutex::new(Database::new())),
            sender: s,
            receiver: r,
        }
    }

    pub fn get_sender(&self) -> Sender<UpdateQuery> {
        Sender::clone(&self.sender)
    }

    pub fn get_db(&self) -> Arc<Mutex<Database>> {
        Arc::clone(&self.data)
    }

    pub async fn run(mut self) {
        log("db start!");

        while let Some(value) = self.receiver.recv().await {
            let mut data = self.data.lock().await;

            match value {
                UpdateQuery::GameData(s) => {
                    data.add_game_result(s);
                }
                UpdateQuery::UserData(s) => {
                    data.add_user_data(s);
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
}
