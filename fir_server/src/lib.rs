mod cli;
mod database;
mod game;
mod game_queue;
mod match_queue;
mod thread_pool;
mod http_handler;
pub mod utility;

pub mod prelude {
    pub use super::utility::log;
}
use self::prelude::*;

/// make threads and run
pub async fn run() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    log("Server start!");

    let db = database::Database::new();
    let db_sender = db.get_sender();
    let real_db = db.get_db();

    let gameq = game_queue::GameQueue::new();
    let game_sender = gameq.get_sender();

    let mut matchq = match_queue::MatchQueue::new();
    let sender = matchq.get_sender();

    let server_handle = tokio::spawn(http_handler::run_server(sender));
    let match_queue_handle = tokio::spawn(match_queue::MatchQueue::run(matchq));
    let game_queue_handle = tokio::spawn(gameq.run(db_sender.clone()));
    let db_handle = tokio::spawn(db.run());
    let cli_handle = tokio::spawn(cli::run(db_sender.clone(), real_db));

    let _ = server_handle.await?;
    match_queue_handle.await?;
    game_queue_handle.await?;
    db_handle.await?;
    cli_handle.await?;

    log("Server end!");
    Ok(())
}
