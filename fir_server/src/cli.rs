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
        UserInfo::new(
            String::from("Alice"),
            String::from("1234"),
            100,
        ),
        UserInfo::new(
            String::from("Jonathan"),
            String::from("qwerty"),
            200,
        ),
    ];
    let game_info = vec![
        GameInfo {
            black_user: user_info[0].clone(),
            white_user: user_info[1].clone(),
            result: GameResult::Draw,
            time: TimeControl {
                seconds: 300,
                fisher: 0,
            },
            notation: vec![
                Coord { x: 5, y: 5 },
                Coord { x: 5, y: 6 },
                Coord { x: 6, y: 5 },
                Coord { x: 4, y: 6 },
                Coord { x: 7, y: 7 },
                Coord { x: 4, y: 7 },
            ],
        },
        GameInfo {
            black_user: user_info[1].clone(),
            white_user: user_info[0].clone(),
            result: GameResult::Win(Side::Black),
            time: TimeControl {
                seconds: 60,
                fisher: 1,
            },
            notation: vec![
                Coord { x: 5, y: 5 },
                Coord { x: 5, y: 6 },
                Coord { x: 6, y: 5 },
                Coord { x: 4, y: 6 },
                Coord { x: 7, y: 7 },
                Coord { x: 4, y: 7 },
            ],
        },
    ];

    for info in user_info {
        tx.send(UpdateQuery::UserInfo(info)).await?;
    }

    for info in game_info {
        tx.send(UpdateQuery::GameInfo(info)).await?;
    }

    Ok(())
}
