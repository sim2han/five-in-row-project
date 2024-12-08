use fir_server;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = tokio::spawn(fir_server::run()).await?;
    Ok(())
}
