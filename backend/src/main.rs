use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    backend::run().await
}
