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
use models::interface::{
    AddUserRequest,
    SetCoupleRequest,
};
use models::data::{
    Couple,
    CoupleID,
    Password,
    Username,
    UserID,
};


#[tokio::main]
async fn main() -> Result<()> {
    LogService::init();

    let config = Config::from_env();
    info!("{:#?}", config);

    let sqlite = SqliteInterface::new(&config).await?;
    let mut users = vec![];
    let data = [("irith", "testpassword"), ("vickivic", "testpassword")];

    for (u, p) in data.iter() {
        let username = Username::new(u)?;
        let password = Password::new(p)?;
        let add_user_request = AddUserRequest::new(username, password);
        users.push(sqlite.add_user(&add_user_request).await?);
    }
    info!("{:?}", users);

    let set_couple_request = SetCoupleRequest::new(UserID::from(1), UserID::from(2));
    let partner = sqlite.set_couple(&set_couple_request).await?;
    info!("{:?}", partner);

    sqlite.remove_user(UserID::from(1)).await?;

    Ok(())
}
