//! Author: irith
//! Date: 2025-11-03 @ 1:51pm
//! Description: The persistent storage component, what data we want to
//! save long-term.

use argon2::{
    Algorithm,
    Argon2,
    Params,
    Version,
};
use argon2::password_hash::{
    rand_core::OsRng,
    PasswordHasher,
    SaltString,
};
use bincode::{
    Decode,
    Encode,
};
use derive_getters::Getters;
use http::StatusCode;
use jiff::Zoned;
use serde::Deserialize;
use tracing::error;
use thiserror::Error;

use std::ops::Range;

use crate::models::interface::HTTPErrorResponse;
use crate::constants::{
    get_username_regex,
    MIN_PASSWORD_LEN,
};
use util_macros::{
    AutoIntoResponse,
    AutoNew,
};


// --- users ---

/// A single user. `password` is a (salted) Argon2 hash. See individual
/// field types for constraints. 
///
/// See the [Ultimate Guide to Rust
/// Newtypes](https://www.howtocodeit.com/articles/ultimate-guide-rust-newtypes)
/// for why we do this. Fields that need to be validated are newtyped.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Getters, AutoNew)]
pub struct User {
    #[getter(copy)]
    id: i64,
    username: Username,
    password: Password,
    // created_at: Zoned,   // TODO: this + additional metadata?
    preferences: UserPreferences,
}


/// Unique identifier for a user, publicly visible.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Username(String);

impl Username {
    /// Canonicalize given username and validate it meets standards.
    pub fn new(raw: &str) -> Result<Self, InvalidUsernameError> {
        let username_re = get_username_regex();
        let username = raw.trim().to_lowercase();
        if username_re.is_match(&username) {
            Ok(Self(username))
        } else {
            Err(InvalidUsernameError(username))
        }
    }

    /// Get a reference to the inner value for encoding purposes.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}


/// Error for not meeting username standards.
///
/// We use `AutoIntoResponse` to make it easy to propagate an error up and
/// out of the app. For example, if `Username` validation fails, we
/// propagate this error type back up a few layers to the HTTP handler,
/// which uses the `IntoResponse` implementation to return a 422 status
/// code with that same error message defined below.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error, AutoIntoResponse)]
#[error("username does not meet minimum requirements: `{0}`")]
#[auto_into_response(StatusCode::UNPROCESSABLE_ENTITY, true)]
pub struct InvalidUsernameError(String);


/// A (salted) Argon2 hash.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Password(String);

impl Password {
    /// Validate a given password meets standards, then hash.
    pub fn new(raw: &str) -> Result<Self, PasswordError> {
        let is_min_len = raw.len() >= MIN_PASSWORD_LEN;
        // TODO: add (optional) check for common passwords.
        if !is_min_len {
            return Err(PasswordError::Length);
        }

        let argon2 = Argon2::new(
            Algorithm::default(),
            Version::default(),
            Params::default(),
            // TODO: move outside and add pepper.
        );
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2.hash_password(raw.as_bytes(), &salt)
                         .map_err(|e| PasswordError::HashFailure(e.to_string()))?
                         .to_string();

        Ok(Self(hash))
    }

    /// Get a reference to the inner value for encoding purposes.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}


/// Errors for not meeting password standards or hashing issues.
///
/// This `AutoIntoResponse` has a variant where the implementation has
/// `is_transparent` set to `false`, so instead of using the error message
/// associated with that variant, we obscure it and instead use a generic
/// message. We don't want to give our users long error traces about how
/// the database query had a syntax error, for example (what're they gonna
/// do with that info?), but we do still want our admin to be able to
/// debug it :).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error, AutoIntoResponse)]
pub enum PasswordError {
    #[error("password does not meet minimum length requirements")]
    #[auto_into_response(StatusCode::UNPROCESSABLE_ENTITY, true)]
    Length,

    #[error("could not create password hash: {0}")]
    #[auto_into_response(StatusCode::INTERNAL_SERVER_ERROR, false)]
    HashFailure(String),
}


/// A set of preferences for features/functionality for a user. We
/// intentionally disable all features by default. We want everything to
/// be OPT-IN (both when the user creates an account AND as new features
/// get added, no surprise data collection add-ins 3 years down the line
/// that the user implicitly "consents" to). :)
///
/// Intentionally chosen not to wrap each field in a newtype (nested raw
/// types + tradeoff of additional complexity for benefits not worth at
/// time of writing).
///
/// While we would LOVE to use `usize` here and everywhere else, encoding
/// into the sqlite DB makes that tricky, so until you find a valid
/// use-case for keeping more than 4_294_967_295 records, we're stuck with
/// this.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Getters)]
pub struct UserPreferences {
    #[getter(copy)]
    new_feature_notifications: bool,

    #[getter(copy)]
    location_history: u32,  // TODO: validated field, should be newtyped?

    #[getter(copy)]
    heartbeat_history: u32,
}

impl UserPreferences {
    /// Create a default set of user preferences. Again, every preference
    /// is intentionally off by default/opt-in only.
    pub fn new() -> Self {
        Self {
            new_feature_notifications: false,
            location_history: 0,
            heartbeat_history: 0,
        }
    }
}


// --- couples ---

/// A paired set of users. Foundation for everything else (Q&A, location
/// sharing, heartbeats, etc).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Getters, AutoNew)]
pub struct Couple {
    #[getter(copy)]
    id: i64,

    #[getter(copy)]
    user_id_1: i64,

    #[getter(copy)]
    user_id_2: i64,
    // created_at: Zoned,
}


// --- questions ---

/// A single prompt template. Note that `answer_type` is for deciding
/// what type of data the corresponding `Answer`s should store/for
/// giving a user the correct way to answer a given question. See
/// `AnswerType` for more details.
#[derive(Debug, Clone, PartialEq, Eq, Hash, AutoNew)]
pub struct Question {
    id: i64,
    category: QuestionCategory,
    prompt: String,
    answer_type: AnswerType,
}


/// A label for grouping questions/what type of topic.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Encode)]
#[serde(rename_all = "snake_case")]
pub enum QuestionCategory {
    General,
}


/// How a user can respond/what type of input to provide. Note an instance
/// of `AnswerType` *only* sets the boundaries of how to respond (e.g.
/// a user should respond with a number between 1 and 5, or free-form
/// input to type anything they want). In other words, this is intended as
/// a marker, not as a content store.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Encode)]
#[serde(rename_all = "snake_case")]
pub enum AnswerType {
    Text,
    YesNo,
    Number(Range<usize>),
    MultipleChoice(Vec<String>),
}


// --- answers ---

/// A single response to a specified question, i.e. this **user**'s
/// response to this question, not both users/partners to the question.
/// Note that `response` is the actual content of the reply/the data,
/// matched to what type of response is specified in `answer_type` of
/// the `Question`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AutoNew)]
pub struct Answer {
    id: i64,
    question_id: i64,
    user_id: i64,
    timestamp: Zoned,
    content: AnswerContent,
}


// TODO: what to do about this duplication/will it become complicated to
// handle?
/// What the user responded to a question with/the actual input provided.
/// 1-to-1 with `AnswerType`'s options.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Encode)]
#[serde(rename_all = "snake_case")]
pub enum AnswerContent {
    Text(String),
    YesNo(bool),
    Number(usize),
    MultipleChoice(usize),
}


// --- locations ---

/// A single location datapoint (of a specific user at a specified instant
/// in time).
#[derive(Debug, Clone, PartialEq, PartialOrd, AutoNew)]
pub struct Location {
    id: i64,
    user_id: i64,
    timestamp: Zoned,
    latitude: Latitude,
    longitude: Longitude,
    accuracy: u32,
}


/// North-south component of location datapoint.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Latitude(f64);

impl Into<f64> for Latitude {
    fn into(self) -> f64 {
        self.0
    }
}


/// West-east component of location datapoint.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Longitude(f64);


impl Into<f64> for Longitude {
    fn into(self) -> f64 {
        self.0
    }
}

// --- heartbeats ---

/// A time range describing when a user is sending their heartbeat to
/// their partner. When they start holding down, we start the range, and
/// end when they lift their finger.
///
/// As a range instead of a single on/off, we accomplish two (basically
/// three) things:
///   1. (basic functionality of if they're sending their heartbeat right
///   now or not, obviously)
///   2. a history of heartbeats, not just "is it happening right now?"
///   3. de-duplication of events/notifications. If they press it 4 times
///   in 1 second, we shouldn't propagate their neediness in 4
///   notifications to their partner (we notify, we don't enable).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, AutoNew)]
pub struct Heartbeat {
    id: i64,
    user_id: i64,
    start_timestamp: Zoned,
    end_timestamp: Option<Zoned>,
}
