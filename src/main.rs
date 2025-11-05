//! Author: irith
//! Date: 2025-11-03 @ 1:51pm
//! Description: Starts up services (bootstrapping with corresponding
//!              interfaces). Just setup/kicking off the backend, nothing
//!              else.

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

use interfaces::DatabaseInterface;
use models::interface::AddUserRequest;
use models::data::{
    Username,
    Password,
};


#[tokio::main]
async fn main() -> Result<()> {
    LogService::init();

    let config = Config::from_env();
    info!("{:#?}", config);

    let sqlite = SqliteInterface::new(&config).await?;
    let username = Username::new("irith")?;
    let password = Password::new("testpassword")?;
    let add_user_request = AddUserRequest::new(username, password);
    let user = sqlite.add_user(&add_user_request).await?;
    info!("{:?}", user);

    Ok(())
}
