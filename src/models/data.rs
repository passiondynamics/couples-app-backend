//! Author: irith
//! Date: 2025-11-03 @ 1:51pm
//! Description: TODO

use jiff::Zoned;

use std::ops::Range;


// --- users ---

/// See the [Ultimate Guide to Rust
/// Newtypes](https://www.howtocodeit.com/articles/ultimate-guide-rust-newtypes)
/// for why we do this. IDs and fields that need to be validated are
/// newtyped.
pub struct User {
    id: UserID,
    username: Username,
    password: Password,
    // created_at: Zoned,   // TODO: this + additional metadata?
    preferences: UserPreferences,
}

impl User {
    pub fn new(id: UserID, username: Username, password: Password) -> Self {
        let preferences = UserPreferences::new();
        Self {id, username, password, preferences}
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserID(usize);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Username(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Password(String);

/// Intentionally chosen not to wrap each field in a newtype (nested raw
/// types + tradeoff of additional complexity for benefits not worth at
/// time of writing).
pub struct UserPreferences {
    new_feature_notifications: bool,
    location_history: usize,    // TODO: validated field, should be newtyped?
    heartbeat_history: usize,
}

impl UserPreferences {
    pub fn new() -> Self {
        // Intentionally disable all features by default. We want
        // everything to be OPT-IN (both when the user creates an account
        // AND as new features get added, no surprise data collection
        // add-ins 3 years down the line that the user consents to
        // implicitly). :)
        Self {
            new_feature_notifications: false,
            location_history: 0,
            heartbeat_history: 0,
        }
    }
}


// --- partners ---

pub struct Partner {
    id: PartnerID,
    user_id_1: UserID,
    user_id_2: UserID,
    // created_at: Zoned,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PartnerID(usize);


// --- questions ---

pub struct Question {
    id: QuestionID,
    category: QuestionCategory,
    prompt: String,
    response_type: ResponseType,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuestionID(usize);

pub enum QuestionCategory {}

pub enum ResponseType {
    Text,
    YesNo,
    Number(Range<usize>),
    MultipleChoice(Vec<String>),
}


// --- answers ---

pub struct Answer {
    id: AnswerID,
    question_id: QuestionID,
    user_id: UserID,
    timestamp: Zoned,
    response: Response,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnswerID(usize);

// TODO: what to do about this duplication/will it become complicated to
// handle?
pub enum Response {
    Text(String),
    YesNo(bool),
    Number(usize),
    MultipleChoice(usize),
}


// --- locations ---

/// Intentionally chosen not to newtype (`Location`s will be passed around
/// atomically + tradeoff of complexity).
pub struct Location {
    id: LocationID,
    user_id: UserID,
    timestamp: Zoned,
    latitude: Latitude,
    longitude: Longitude,
    accuracy: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocationID(usize);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Latitude(f64);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Longitude(f64);


// --- heartbeats ---

pub struct Heartbeat {
    id: HeartbeatID,
    user_id: UserID,
    start_timestamp: Zoned,
    end_timestamp: Option<Zoned>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeartbeatID(usize);
