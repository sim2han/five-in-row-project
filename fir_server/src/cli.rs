use tokio::sync::Mutex;

use crate::database::{data::*, DbSender, RealData, UpdateQuery};
use crate::prelude::*;
use std::io;
use std::sync::Arc;

pub async fn run(
    tx: DbSender,
    db: Arc<Mutex<RealData>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let command = buf
            .split_whitespace()
            .map(|str| String::from(str))
            .collect::<Vec<String>>();

        log(&format!("command {}", command[0]));

        if command[0] == "exit" {
            break;
        } else if command[0] == "sample" {
            let txc = tx.clone();
            let _ = tokio::spawn(async move {
                add_sample_datas(txc).await.unwrap();
            })
            .await;
        } else if command[0] == "getdb" {
            let data = db.lock().await;
            let user_data = data.get_all_user();
            println!("{user_data:?}");
        } else {
            log(&format!("unkown command {}", command[0]));
        }
    }
    Ok(())
}

async fn add_sample_datas(tx: DbSender) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_info = vec![vec!["Alice", "1234"], vec!["Jonathan", "qwerty"]];

    for info in user_info {
        tx.send(UpdateQuery::UserInfo(UserInfo {
            id: String::from(info[0]),
            pwd: String::from(info[1]),
            rating: 10,
        }))
        .await?;
    }

    Ok(())
}
