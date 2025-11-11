//! Author: irith
//! Date: 2025-11-03 @ 6:59pm
//! Description: Describes what changes can be made to the data through
//! the interfaces (and what info is needed to make those changes).

use thiserror::Error;
use derive_getters::Getters;
use jiff::Zoned;

use crate::models::data::{
    Latitude,
    Longitude,
    Password,
    QuestionCategory,
    Response,
    ResponseType,
    Username,
};


/// Request used to add a new user.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Getters)]
pub struct AddUserRequest {
    username: Username,
    password: Password,
}

impl AddUserRequest {
    pub fn new(username: Username, password: Password) -> Self {
        Self {username, password}
    }
}


/// Errors when adding a user.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum AddUserError {
    #[error("username already exists: `{0}`")]
    UsernameExists(String),

    #[error("could not insert user: {0}")]
    Unknown(String),
}


/// Errors when removing a user.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum RemoveUserError {
    #[error("could not find corresponding user")]
    UserNotFound,

    #[error("could not remove user: {0}")]
    Unknown(String),
}


/// Request to associate two users together.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Getters)]
pub struct SetCoupleRequest {
    #[getter(copy)]
    user_id_1: i64,

    #[getter(copy)]
    user_id_2: i64,
}

impl SetCoupleRequest {
    pub fn new(user_id_1: i64, user_id_2: i64) -> Self {
        Self {user_id_1, user_id_2}
    }
}


/// Errors when associating users together.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum SetCoupleError {
    #[error("could not find corresponding user")]
    UserNotFound,

    #[error("could not set couple: {0}")]
    Unknown(String),
}


/// Errors when disassociating users from each other.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum UnsetCoupleError {
    #[error("could not find corresponding couple")]
    CoupleNotFound,

    #[error("could not unset couple: {0}")]
    Unknown(String),
}


/// TODO
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddQuestionRequest {
    category: QuestionCategory,
    prompt: String,
    response_type: ResponseType,
}

impl AddQuestionRequest {
    pub fn new(category: QuestionCategory, prompt: String, response_type: ResponseType) -> Self {
        Self {category, prompt, response_type}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum AddQuestionError {}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum RemoveQuestionError {}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddAnswerRequest {
    question_id: i64,
    user_id: i64,
    timestamp: Zoned,
    response: Response,
}

impl AddAnswerRequest {
    pub fn new(question_id: i64, user_id: i64, timestamp: Zoned, response: Response) -> Self {
        Self {question_id, user_id, timestamp, response}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum AddAnswerError {}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AddLocationRequest {
    user_id: i64,
    timestamp: Zoned,
    latitude: Latitude,
    longitude: Longitude,
    accuracy: usize,
}

impl AddLocationRequest {
    pub fn new(user_id: i64, timestamp: Zoned, latitude: Latitude, longitude: Longitude, accuracy: usize) -> Self {
        Self {user_id, timestamp, latitude, longitude, accuracy}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum AddLocationError {}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StartHeartbeatRequest {
    user_id: i64,
    start_timestamp: Zoned,
}

impl StartHeartbeatRequest {
    pub fn new(user_id: i64, start_timestamp: Zoned) -> Self {
        Self {user_id, start_timestamp}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum StartHeartbeatError {}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndHeartbeatRequest {
    heartbeat_id: i64,
    end_timestamp: Zoned,
}

impl EndHeartbeatRequest {
    pub fn new(heartbeat_id: i64, end_timestamp: Zoned) -> Self {
        Self {heartbeat_id, end_timestamp}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum EndHeartbeatError {}
