//! Author: irith
//! Date: 2025-11-10 @ 7:43pm
//! Description: TODO

use axum::extract::State;
use axum::response::{
    IntoResponse,
    Result,
};
use http::StatusCode;

use crate::services::AppService;
use crate::interfaces::http::AppState;


// --- TODO: temporary ---
use crate::interfaces::database::DatabaseInterface;
use crate::models::interface::{
    AddUserRequest,
    SetCoupleRequest,
};
use crate::models::data::{
    Couple,
    Password,
    Username,
};
// --- TODO: temporary ---


pub async fn health() -> &'static str {
    "I'm healthy!"
}

/// We go from:
/// ```
/// pub async fn add_user() -> &'static str {
/// ```
/// to (returning any type of response, e.g. a body or a status code in
/// the same function),
/// ```
/// pub async fn add_user() -> impl IntoResponse {
/// ```
/// to (using our global state, with the extra generics needed for it)
/// ```
/// pub async fn add_user<A: AppService>(
///     State(state): State<AppState<A>>
/// ) -> impl IntoResponse {
/// ```
/// to (using `?` to nicely unwrap any `Result`s, using `axum`'s wrapper
/// `Result`)
/// ```
/// pub async fn add_user<A: AppService>(
///     State(state): State<AppState<A>>
/// ) -> Result<impl IntoResponse> {
/// ```
pub async fn add_user<A: AppService>(
    State(state): State<AppState<A>>,
) -> Result<impl IntoResponse> {
    let username = Username::new("irh")?;
    /*
    let password = Password::new("testpassword")?;
    let add_user_request = AddUserRequest::new(username, password);

    state.app.add_user(&add_user_request).await?;
    Err::<&str, StatusCode>(StatusCode::FORBIDDEN)
    */

    Ok("Added user!")
}
