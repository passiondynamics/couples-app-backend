//! Author: irith
//! Date: 2025-11-10 @ 7:43pm
//! Description: TODO

use axum::{
    Router,
    extract::{
        Json,
        Query,
        State,
    },
    response::{
        IntoResponse,
        Result,
    },
    routing::{
        delete,
        get,
        post,
//        put,
    },
};
use http::StatusCode;

use crate::models::interface::{
    AddUserRequest,
    HTTPResponse,
    RawAddUserRequest,
    RemoveUserRequest,
};
use crate::interfaces::http::AppState;
use crate::services::AppService;


pub fn generate_routes<A: AppService>() -> Router<AppState<A>> {
    Router::new()
           .route("/health", get(health))
           .route("/user", post(add_user).delete(remove_user))
}


async fn health() -> &'static str {
    "I'm healthy!"
}

/// TODO
///
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
/// to (actually taking a body):
async fn add_user<A: AppService>(
    State(state): State<AppState<A>>,
    Json(body): Json<RawAddUserRequest>,
) -> Result<impl IntoResponse> {
    let request = AddUserRequest::try_from(body)?;
    let user = state.app.add_user(&request).await?;

    Ok(StatusCode::CREATED)
}

async fn remove_user<A: AppService>(
    State(state): State<AppState<A>>,
    request: Query<RemoveUserRequest>,
) -> Result<impl IntoResponse> {
    state.app.remove_user(&request).await?;

    Ok(StatusCode::NO_CONTENT)
}

