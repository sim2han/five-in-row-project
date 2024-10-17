use tokio::sync::Mutex;

use crate::prelude::*;
use crate::database::{DbSender, UpdateQuery, UserInfo, RealData};
use std::io;
use std::sync::Arc;

pub async fn run(mut tx: DbSender, mut db: Arc<Mutex<RealData>>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let buf = String::from(buf.trim());
        
        if buf == "Hello" {
            log("helooo");
        }
        else if buf == "exit" {
            todo!();
        }
        else if buf == "sample" {
            add_sample_datas(&mut tx).await?;
        }
        else if buf == "getdb" {
            let mut db = db.lock().await;
            if let Some(data) = db.get_user_data(String::from("AAAA")) {
                log(&format!("{:?}", data));
            }
            else {
                log("cannot find aaaa");
            }
        }
        else {
            log(buf.as_str());
        }
    }
}

async fn add_sample_datas(tx: &mut DbSender) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tx.send(UpdateQuery::UserInfo(
        UserInfo::from_username(String::from("AAAA"))
    )).await?;
    Ok(())
}