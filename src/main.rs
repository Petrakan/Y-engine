use clean_rust::start_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    start_app().await?;
    Ok(())
}
