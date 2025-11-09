//! Author: irith
//! Date: 2025-11-03 @ 1:51pm
//! Description: Starts up services (bootstrapping with corresponding
//! interfaces). Just setup/kicking off the backend, nothing else.

use anyhow::Result;
use tracing::info;

mod constants;
mod interfaces;
use interfaces::database::SQLiteInterface;
use interfaces::http::HTTPInterface;
mod models;
use models::config::{
    Config,
};
mod services;
use services::{
    LogService,
    CouplesAppService,
};

use interfaces::database::DatabaseInterface;
use models::interface::{
    AddUserRequest,
    SetCoupleRequest,
};
use models::data::{
    Couple,
    Password,
    Username,
};


/// Set up application from the inside -> outward, and start.
#[tokio::main]
async fn main() -> Result<()> {
    // Miscellaneous components, independent of the main flow of data.
    LogService::init();

    let config = Config::from_env();
    info!("{:#?}", config);

    // Main flow of data (http -> core service -> database); we initialize
    // from the bottom upwards (in other words, at the innermost and work
    // our way out).
    let sqlite = SQLiteInterface::new(&config).await?;
    let app = CouplesAppService::new(sqlite);
    let http = HTTPInterface::new(&config, app).await?;

    // --- TODO: temporary ---
    /*

    let mut users = vec![];
    let data = [("irith", "testpassword"), ("vickivic", "testpassword")];

    for (u, p) in data.iter() {
        let username = Username::new(u)?;
        let password = Password::new(p)?;
        let add_user_request = AddUserRequest::new(username, password);
        users.push(sqlite.add_user(&add_user_request).await?);
    }
    info!("{:?}", users);

    let set_couple_request = SetCoupleRequest::new(1, 2);
    let partner = sqlite.set_couple(&set_couple_request).await?;
    info!("{:?}", partner);

    sqlite.remove_user(1).await?;

    */
    // --- TODO: temporary ---

    Ok(())
}
