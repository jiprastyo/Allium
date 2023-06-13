#![feature(async_fn_in_trait)]

mod allium_menu;
mod command;
mod menu;

use anyhow::Result;

use allium_menu::AlliumMenu;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let mut app = AlliumMenu::new()?;
    app.run_event_loop().await?;
    Ok(())
}
