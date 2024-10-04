use fir_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tokio::spawn(fir_server::run()).await?;
    Ok(())
}
