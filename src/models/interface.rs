//! Author: irith
//! Date: 2025-11-03 @ 6:59pm
//! Description: TODO

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


pub struct AddUserRequest {
    username: Username,
    password: Password,
}

impl AddUserRequest {
    pub fn new(username: Username, password: Password) -> Self {
        Self {username, password}
    }
}


pub enum AddUserError {}


pub enum RemoveUserError {}


pub struct AddPartnerRequest {
    user_id_1: UserID,
    user_id_2: UserID,
}

impl AddPartnerRequest {
    pub fn new(user_id_1: UserID, user_id_2: UserID) -> Self {
        Self {user_id_1, user_id_2}
    }
}


pub enum AddPartnerError {}


pub enum RemovePartnerError {}


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


pub enum AddQuestionError {}


pub enum RemoveQuestionError {}


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


pub enum AddAnswerError {}


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


pub enum AddLocationError {}


pub struct StartHeartbeatRequest {
    user_id: UserID,
    start_timestamp: Zoned,
}

impl StartHeartbeatRequest {
    pub fn new(user_id: UserID, start_timestamp: Zoned) -> Self {
        Self {user_id, start_timestamp}
    }
}


pub enum StartHeartbeatError {}


pub struct EndHeartbeatRequest {
    heartbeat_id: HeartbeatID,
    end_timestamp: Zoned,
}

impl EndHeartbeatRequest {
    pub fn new(heartbeat_id: HeartbeatID, end_timestamp: Zoned) -> Self {
        Self {heartbeat_id, end_timestamp}
    }
}


pub enum EndHeartbeatError {}
