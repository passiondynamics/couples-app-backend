//! Author: irith
//! Date: 2025-11-03 @ 1:51pm
//! Description: TODO

use anyhow::Result;
use tracing::info;

mod constants;
mod interfaces;
use interfaces::SqliteInterface;
mod models;
use models::config::{
    Config,
};
mod services;
use services::{
    LogService,
};


#[tokio::main]
async fn main() -> Result<()> {
    LogService::init();

    let config = Config::from_env();
    info!("{:#?}", config);

    let sqlite = SqliteInterface::new(&config).await?;

    Ok(())
}
