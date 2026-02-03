//! Author: irith
//! Date: 2025-11-03 @ 6:59pm
//! Description: Describes what changes can be made to the data through
//! the interfaces (and what info is needed to make those changes).

use axum::response::IntoResponse;
use derive_getters::Getters;
use http::StatusCode;
use jiff::Zoned;
use serde::{
    Deserialize,
    Serialize,
};
use thiserror::Error;

use crate::models::data::{
    AnswerContent,
    AnswerType,
    Latitude,
    Longitude,
    Password,
    QuestionCategory,
    Username,
};
use util_macros::{
    AutoDeserialize,
    AutoIntoResponse,
    AutoNew,
};


// --- http only ---

macro_rules! generate_response {
    ($name:ident, $( $d:ident ),*) => {
        #[derive(Debug, Clone, Serialize)]
        pub struct $name<T>
        where
            T: IntoResponse,
        {
            $(
                $d: T,
            )*
        }

        impl<T> $name<T>
        where
            T: IntoResponse,
        {
            pub fn new($($d: T)*) -> Self {
                Self {$($d)*}
            }
        }
    }
}

generate_response!(HTTPResponse, data);
generate_response!(HTTPErrorResponse, error);


// --- http + database ---

/// Request used to add a new user.
///
/// We use `AutoDeserialize` here since we have field types that have
/// validation logic. That generated `try_from` method propagates errors
/// thrown by each of those types when validation fails (for example, if
/// validation fails for making a new `Username`, `try_from` propagates an
/// `InvalidUsernameError`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Getters, AutoNew, AutoDeserialize)]
pub struct AddUserRequest {
    #[auto_deserialize(String)]
    username: Username,
    #[auto_deserialize(String)]
    password: Password,
}


/// Errors when adding a user.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error, AutoIntoResponse)]
pub enum AddUserError {
    #[error("username already exists: `{0}`")]
    #[auto_into_response(StatusCode::CONFLICT, true)]
    UsernameExists(String),

    #[error("could not insert user into database: {0}")]
    #[auto_into_response(StatusCode::INTERNAL_SERVER_ERROR, false)]
    InsertFailure(String),
}


/// Request to remove an existing user.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Getters, AutoNew)]
pub struct RemoveUserRequest {
    #[getter(copy)]
    user_id: i64,
}


/// Errors when removing a user.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error, AutoIntoResponse)]
pub enum RemoveUserError {
    #[error("could not find corresponding user")]
    #[auto_into_response(StatusCode::NOT_FOUND, true)]
    UserNotFound,

    #[error("could not delete user from database: {0}")]
    #[auto_into_response(StatusCode::INTERNAL_SERVER_ERROR, false)]
    DeleteFailure(String),
}


/// Request to associate two users together.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Getters, AutoNew)]
pub struct SetCoupleRequest {
    #[getter(copy)]
    user_id_1: i64,

    #[getter(copy)]
    user_id_2: i64,
}


/// Errors when associating users together.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error, AutoIntoResponse)]
pub enum SetCoupleError {
    #[error("could not find corresponding user for couple")]
    #[auto_into_response(StatusCode::NOT_FOUND, true)]
    UserNotFound,

    #[error("could not insert couple into database: {0}")]
    #[auto_into_response(StatusCode::INTERNAL_SERVER_ERROR, false)]
    InsertFailure(String),
}


/// Request to disassociate users from each other.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Getters, AutoNew)]
pub struct UnsetCoupleRequest {
    #[getter(copy)]
    couple_id: i64,
}


/// Errors when disassociating users from each other.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error, AutoIntoResponse)]
pub enum UnsetCoupleError {
    #[error("could not find corresponding couple")]
    #[auto_into_response(StatusCode::NOT_FOUND, true)]
    CoupleNotFound,

    #[error("could not delete couple from database: {0}")]
    #[auto_into_response(StatusCode::INTERNAL_SERVER_ERROR, false)]
    DeleteFailure(String),
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Getters, AutoNew)]
pub struct AddQuestionRequest {
    #[getter(copy)]
    category: QuestionCategory,
    prompt: String,
    answer_type: AnswerType,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error, AutoIntoResponse)]
pub enum AddQuestionError {
    #[error("could not encode question field: {0}")]
    #[auto_into_response(StatusCode::INTERNAL_SERVER_ERROR, false)]
    EncodeFailure(String),

    #[error("could not insert question into database: {0}")]
    #[auto_into_response(StatusCode::INTERNAL_SERVER_ERROR, false)]
    InsertFailure(String),
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Getters, AutoNew)]
pub struct RemoveQuestionRequest {
    #[getter(copy)]
    question_id: i64,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error, AutoIntoResponse)]
pub enum RemoveQuestionError {
    #[error("could not find corresponding question")]
    #[auto_into_response(StatusCode::NOT_FOUND, true)]
    QuestionNotFound,

    #[error("could not delete question from database: {0}")]
    #[auto_into_response(StatusCode::INTERNAL_SERVER_ERROR, false)]
    DeleteFailure(String),
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Getters, AutoNew)]
pub struct AddAnswerRequest {
    #[getter(copy)]
    question_id: i64,

    #[getter(copy)]
    user_id: i64,
    timestamp: Zoned,
    content: AnswerContent,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error, AutoIntoResponse)]
pub enum AddAnswerError {
    #[error("could not encode answer field: {0}")]
    #[auto_into_response(StatusCode::INTERNAL_SERVER_ERROR, false)]
    EncodeFailure(String),

    #[error("could not find corresponding question or user for answer")]
    #[auto_into_response(StatusCode::NOT_FOUND, true)]
    QuestionOrUserNotFound,


    #[error("could not insert answer into database: {0}")]
    #[auto_into_response(StatusCode::INTERNAL_SERVER_ERROR, false)]
    InsertFailure(String),
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Getters, AutoNew)]
pub struct AddLocationRequest {
    #[getter(copy)]
    user_id: i64,
    timestamp: Zoned,

    #[getter(copy)]
    latitude: Latitude,

    #[getter(copy)]
    longitude: Longitude,

    #[getter(copy)]
    accuracy: u32,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error, AutoIntoResponse)]
pub enum AddLocationError {
    #[error("could not find corresponding user for location")]
    #[auto_into_response(StatusCode::NOT_FOUND, true)]
    UserNotFound,


    #[error("could not insert location into database: {0}")]
    #[auto_into_response(StatusCode::INTERNAL_SERVER_ERROR, false)]
    InsertFailure(String),
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Getters, AutoNew)]
pub struct StartHeartbeatRequest {
    #[getter(copy)]
    user_id: i64,
    start_timestamp: Zoned,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error, AutoIntoResponse)]
pub enum StartHeartbeatError {
    #[error("could not find corresponding user for heartbeat")]
    #[auto_into_response(StatusCode::NOT_FOUND, true)]
    UserNotFound,


    #[error("could not insert heartbeat into database: {0}")]
    #[auto_into_response(StatusCode::INTERNAL_SERVER_ERROR, false)]
    InsertFailure(String),
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AutoNew)]
pub struct EndHeartbeatRequest {
    heartbeat_id: i64,
    end_timestamp: Zoned,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum EndHeartbeatError {}
