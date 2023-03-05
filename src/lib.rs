mod aplication;
mod domain;
mod infrastructure;
mod interfaces;

pub async fn start_app() -> anyhow::Result<()> {
    println!("Hello world");
    Ok(())
}
