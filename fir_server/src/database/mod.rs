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
    users: Vec<data::UserData>,
    games: Vec<data::GameData>,
}

impl Database {
    fn new() -> Self {
        Database {
            users: vec![],
            games: vec![],
        }
    }

    pub fn register_user(&mut self, s: info::RegisterInfo) -> info::UserKeyInfo {
        let key = s.id.clone() + "_key";
        self.users.push(data::UserData {
            id: s.id.clone(),
            pwd: s.pwd,
            rating: 600,
            key: key.clone(),
        });
        info::UserKeyInfo { key: key }
    }

    pub fn add_user_data(&mut self, data: data::UserData) {
        log(&format!("add user data {:?}", data));
        self.users.push(data);
    }

    pub fn add_game_result(&mut self, data: data::GameData) {
        log(&format!("add game result {:?}", data));
        self.games.push(data);
    }

    pub fn try_login(&self, info: &info::LoginInfo) -> Option<data::UserData> {
        for user in self.users.iter() {
            if user.id == info.id && user.pwd == info.pwd {
                return Some(user.clone());
            }
        }
        None
    }

    pub fn get_user(&self, key: &info::UserKeyInfo) -> Option<data::UserData> {
        for info in self.users.iter() {
            if info.key == key.key {
                return Some(info.clone());
            }
        }
        None
    }

    pub fn get_user_game(&self, key: &info::UserKeyInfo) -> Vec<info::GameInfo> {
        self.games
            .iter()
            .filter(|game| game.black_user.key == key.key || game.white_user.key == key.key)
            .map(|game| game.clone().into())
            .collect()
    }

    pub fn get_all_user(&self) -> &Vec<data::UserData> {
        &self.users
    }

    pub fn get_all_game(&self) -> &Vec<data::GameData> {
        &self.games
    }

    pub fn get_all_game_serialize(
        &self,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let str = serde_json::to_string(&self.games)?;
        Ok(str)
    }

    pub fn get_all_user_serizlie(
        &self,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let str = serde_json::to_string(&self.users)?;
        Ok(str)
    }
}

#[derive(Debug)]
pub enum UpdateQuery {
    NewUser(info::RegisterInfo),
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
                UpdateQuery::NewUser(s) => {
                    data.add_user_data(data::UserData {
                        id: s.id.clone(),
                        pwd: s.pwd,
                        rating: 600,
                        key: s.id.clone() + "_key",
                    });
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
}
