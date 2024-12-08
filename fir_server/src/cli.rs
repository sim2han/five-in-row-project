use tokio::sync::Mutex;

use crate::database::data;
use crate::database::data::Notation;
use crate::database::{data::*, Database, DbSender, UpdateQuery};
use crate::prelude::*;
use std::io;
use std::sync::Arc;

pub async fn run(
    tx: DbSender,
    db: Arc<Mutex<Database>>,
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
            tokio::spawn(async move {
                add_sample_datas(txc).await.unwrap();
            })
            .await?;
        } else if command[0] == "printdb" {
            let data = db.lock().await;
            let user_data = data.get_all_user();
            println!("USER INFO");
            println!("{user_data:?}");
            let game_data = data.get_all_game();
            println!("GAME INFO");
            println!("{game_data:?}");
        } else {
            log(&format!("unkown command {}", command[0]));
        }
    }
    Ok(())
}

async fn add_sample_datas(tx: DbSender) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user_info = vec![
        data::UserData {
            id: String::from("Alice"),
            pwd: String::from("1234"),
            rating: 100,
            key: String::from("Alice_key"),
        },
        data::UserData {
            id: String::from("Jonathan"),
            pwd: String::from("qwerty"),
            rating: 200,
            key: String::from("Jonathan_key"),
        },
    ];
    let game_info = vec![
        data::GameData {
            black_user: user_info[0].clone(),
            white_user: user_info[1].clone(),
            result: GameResult::Draw,
            notations: vec![
                Notation {
                    color: true,
                    x: 5,
                    y: 5,
                },
                Notation {
                    color: false,
                    x: 5,
                    y: 6,
                },
                Notation {
                    color: true,
                    x: 6,
                    y: 5,
                },
                Notation {
                    color: false,
                    x: 4,
                    y: 6,
                },
                Notation {
                    color: true,
                    x: 7,
                    y: 7,
                },
                Notation {
                    color: false,
                    x: 4,
                    y: 7,
                },
            ],
        },
        data::GameData {
            black_user: user_info[1].clone(),
            white_user: user_info[0].clone(),
            result: GameResult::Win(Side::Black),
            notations: vec![
                Notation {
                    color: true,
                    x: 5,
                    y: 5,
                },
                Notation {
                    color: false,
                    x: 5,
                    y: 6,
                },
                Notation {
                    color: true,
                    x: 6,
                    y: 5,
                },
                Notation {
                    color: false,
                    x: 4,
                    y: 6,
                },
                Notation {
                    color: true,
                    x: 7,
                    y: 7,
                },
                Notation {
                    color: false,
                    x: 4,
                    y: 7,
                },
                Notation {
                    color: true,
                    x: 1,
                    y: 1,
                },
            ],
        },
    ];

    for info in user_info {
        tx.send(UpdateQuery::UserData(info)).await?;
    }

    for info in game_info {
        tx.send(UpdateQuery::GameData(info)).await?;
    }

    Ok(())
}
