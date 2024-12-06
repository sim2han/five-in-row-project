mod cli;
mod database;
mod game_queue;
mod http_handler;
mod match_queue;
mod socket;
mod utility;

pub mod prelude {
    pub use super::utility::{log, Stopper};
}
use self::prelude::*;
use tokio::spawn;

/// start server
pub async fn run() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    log("server start!");

    let db = database::DataManager::new();
    let dbq = db.get_sender();
    let real_db = db.get_db();

    let game = game_queue::GameQueue::new();
    let gameq = game.get_sender();

    let mut game_match = match_queue::MatchQueue::new();
    let matchq = game_match.get_sender();

    let server_handle = spawn(http_handler::run_server(
        matchq,
        dbq.clone(),
        real_db.clone(),
    ));
    let match_queue_handle = spawn(game_match.run(gameq.clone()));
    let game_queue_handle = spawn(game.run(dbq.clone()));
    let db_handle = spawn(db.run());
    let cli_handle = spawn(cli::run(dbq.clone(), real_db.clone()));

    server_handle.await??;
    match_queue_handle.await?;
    game_queue_handle.await?;
    db_handle.await?;
    cli_handle.await??;

    log("server end!");
    Ok(())
}
