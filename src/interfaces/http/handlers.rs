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
    AddAnswerRequest,
    AddLocationRequest,
    AddQuestionRequest,
    AddUserRequest,
//    HTTPResponse,
    RawAddUserRequest,
    RemoveQuestionRequest,
    RemoveUserRequest,
    SetCoupleRequest,
    StartHeartbeatRequest,
    UnsetCoupleRequest,
};
use crate::interfaces::http::AppState;
use crate::services::AppService;


pub fn generate_routes<A: AppService>() -> Router<AppState<A>> {
    Router::new()
           .route("/health", get(health))
           .route("/user", post(add_user).delete(remove_user))  // TODO: delete goes to `/user/{user_id}` w/ Path extractor?
           .route("/couple", post(set_couple).delete(unset_couple))
           .route("/question", post(add_question).delete(remove_question))
           .route("/answer", post(add_answer))
           .route("/location", post(add_location))
           .route("/heartbeat", post(start_heartbeat))
}


async fn health() -> &'static str {
    "I'm healthy!"
}


/// Create a new user. Here, we HAVE to use the intermediate
/// deserialization struct `RawAddUserRequest`, because we have custom
/// validation in our final struct fields. See `AddUserRequest`'s use of
/// `AutoDeserialize` (and its corresponding implementation in the
/// `util-macros` crate) for more info.
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
    // Convert the intermediate struct into our final struct. If the
    // validation fails, the `try_from` implementation returns an error
    // type that implements `IntoResponse`, so we just pass that back up.
    // See `AutoIntoResponse` on `InvalidUsernameError`, `PasswordError`
    // for more info.
    let request = AddUserRequest::try_from(body)?;

    // The error type here is something similar, it also implements
    // `IntoResponse` (via `AutoIntoResponse`) that we can pass back quite
    // easily.
    let _user = state.app.add_user(&request).await?;

    Ok(StatusCode::CREATED)
}


/// Remove a given user.
async fn remove_user<A: AppService>(
    State(state): State<AppState<A>>,
    request: Query<RemoveUserRequest>,
) -> Result<impl IntoResponse> {
    state.app.remove_user(&request).await?;

    Ok(StatusCode::NO_CONTENT)
}


/// Associate two users together as a couple.
async fn set_couple<A: AppService>(
    State(state): State<AppState<A>>,
    Json(request): Json<SetCoupleRequest>,
) -> Result<impl IntoResponse> {
    state.app.set_couple(&request).await?;

    Ok(StatusCode::CREATED)
}


/// Disassociate the given couple from each other.
async fn unset_couple<A: AppService>(
    State(state): State<AppState<A>>,
    request: Query<UnsetCoupleRequest>,
) -> Result<impl IntoResponse> {
    state.app.unset_couple(&request).await?;

    Ok(StatusCode::NO_CONTENT)
}


/// Add a new question.
async fn add_question<A: AppService>(
    State(state): State<AppState<A>>,
    Json(request): Json<AddQuestionRequest>,
) -> Result<impl IntoResponse> {
    println!("request: {:?}", request);
    state.app.add_question(&request).await?;

    Ok(StatusCode::CREATED)
}


/// Remove a given question.
async fn remove_question<A: AppService>(
    State(state): State<AppState<A>>,
    request: Query<RemoveQuestionRequest>,
) -> Result<impl IntoResponse> {
    state.app.remove_question(&request).await?;

    Ok(StatusCode::NO_CONTENT)
}


/// Add a new answer.
async fn add_answer<A: AppService>(
    State(state): State<AppState<A>>,
    Json(request): Json<AddAnswerRequest>,
) -> Result<impl IntoResponse> {
    state.app.add_answer(&request).await?;

    Ok(StatusCode::CREATED)
}

/// Add a new location datapoint.
async fn add_location<A: AppService>(
    State(state): State<AppState<A>>,
    Json(request): Json<AddLocationRequest>,
) -> Result<impl IntoResponse> {
    state.app.add_location(&request).await?;

    Ok(StatusCode::CREATED)
}

/// Add a new heartbeat range.
async fn start_heartbeat<A: AppService>(
    State(state): State<AppState<A>>,
    Json(request): Json<StartHeartbeatRequest>,
) -> Result<impl IntoResponse> {
    state.app.start_heartbeat(&request).await?;

    Ok(StatusCode::CREATED)
}
/*

/// Mark a given heartbeat range as finished.
*/
