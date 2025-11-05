//! Author: irith
//! Date: 2025-11-03 @ 6:59pm
//! Description: Describes what changes can be made to the data through
//!              the interfaces (and what info is needed to make those
//!              changes).

use thiserror::Error;
use jiff::Zoned;

use crate::models::data::{
    HeartbeatID,
    Latitude,
    Longitude,
    Partner,
    Password,
    QuestionCategory,
    QuestionID,
    Response,
    ResponseType,
    User,
    Username,
    UserID,
};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddUserRequest {
    username: Username,
    password: Password,
}

impl AddUserRequest {
    pub fn new(username: Username, password: Password) -> Self {
        Self {username, password}
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn password(&self) -> &Password {
        &self.password
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum AddUserError {
    #[error("username already exists: `{0}`")]
    UsernameExists(String),

    #[error("could not insert user: {0}")]
    Unknown(String),
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RemoveUserError {}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddPartnerRequest {
    user_id_1: UserID,
    user_id_2: UserID,
}

impl AddPartnerRequest {
    pub fn new(user_id_1: UserID, user_id_2: UserID) -> Self {
        Self {user_id_1, user_id_2}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddPartnerError {}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RemovePartnerError {}


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


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddQuestionError {}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RemoveQuestionError {}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AddAnswerRequest {
    question_id: QuestionID,
    user_id: UserID,
    timestamp: Zoned,
    response: Response,
}

impl AddAnswerRequest {
    pub fn new(question_id: QuestionID, user_id: UserID, timestamp: Zoned, response: Response) -> Self {
        Self {question_id, user_id, timestamp, response}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddAnswerError {}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AddLocationRequest {
    user_id: UserID,
    timestamp: Zoned,
    latitude: Latitude,
    longitude: Longitude,
    accuracy: usize,
}

impl AddLocationRequest {
    pub fn new(user_id: UserID, timestamp: Zoned, latitude: Latitude, longitude: Longitude, accuracy: usize) -> Self {
        Self {user_id, timestamp, latitude, longitude, accuracy}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AddLocationError {}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StartHeartbeatRequest {
    user_id: UserID,
    start_timestamp: Zoned,
}

impl StartHeartbeatRequest {
    pub fn new(user_id: UserID, start_timestamp: Zoned) -> Self {
        Self {user_id, start_timestamp}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StartHeartbeatError {}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndHeartbeatRequest {
    heartbeat_id: HeartbeatID,
    end_timestamp: Zoned,
}

impl EndHeartbeatRequest {
    pub fn new(heartbeat_id: HeartbeatID, end_timestamp: Zoned) -> Self {
        Self {heartbeat_id, end_timestamp}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EndHeartbeatError {}
