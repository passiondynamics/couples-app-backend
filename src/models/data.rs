//! Author: irith
//! Date: 2025-11-03 @ 1:51pm
//! Description: The persistent storage component, what data we want to
//!              save long-term.

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
use jiff::Zoned;
use thiserror::Error;

use std::ops::Range;

use crate::constants::{
    get_username_regex,
    MIN_PASSWORD_LEN,
};


// --- users ---

/// A single user. `password` is a (salted) Argon2 hash. See individual
/// field types for constraints. 
///
/// See the [Ultimate Guide to Rust
/// Newtypes](https://www.howtocodeit.com/articles/ultimate-guide-rust-newtypes)
/// for why we do this. IDs and fields that need to be validated are
/// newtyped.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct User {
    id: UserID,
    username: Username,
    password: Password,
    // created_at: Zoned,   // TODO: this + additional metadata?
    preferences: UserPreferences,
}

impl User {
    /// Create a new user.
    pub fn new(raw_id: i64, username: Username, password: Password, preferences: UserPreferences) -> Self {
        let id = UserID::from(raw_id);
        Self {id, username, password, preferences}
    }

    pub fn id(self) -> UserID {
        self.id
    }
}


/// Unique identifier for a user, for internal use only.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserID(i64);

impl From<i64> for UserID {
    fn from(raw: i64) -> Self {
        Self(raw)
    }
}

impl Into<i64> for UserID {
    fn into(self) -> i64 {
        self.0
    }
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
            Err(InvalidUsernameError)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}


/// Error for not meeting username standards.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("username does not meet minimum requirements")]
pub struct InvalidUsernameError;


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
            // TODO: add pepper.
        );
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2.hash_password(raw.as_bytes(), &salt)
                         .map_err(|e| PasswordError::Unknown(e.to_string()))?
                         .to_string();

        Ok(Self(hash))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Errors for not meeting password standards or hashing issues.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
pub enum PasswordError {
    #[error("password does not meet minimum length requirements")]
    Length,

    #[error("could not create password: {0}")]
    Unknown(String),
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserPreferences {
    new_feature_notifications: bool,
    location_history: u32,  // TODO: validated field, should be newtyped?
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

    pub fn new_feature_notifications(&self) -> bool {
        self.new_feature_notifications
    }

    pub fn location_history(&self) -> u32 {
        self.location_history
    }

    pub fn heartbeat_history(&self) -> u32 {
        self.heartbeat_history
    }
}


// --- couples ---

/// A paired set of users. Foundation for everything else (Q&A, location
/// sharing, heartbeats, etc).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Couple {
    id: CoupleID,
    user_id_1: UserID,
    user_id_2: UserID,
    // created_at: Zoned,
}

impl Couple {
    pub fn new(raw_id: i64, user_id_1: UserID, user_id_2: UserID) -> Self {
        let id = CoupleID::from(raw_id);
        Self {id, user_id_1, user_id_2}
    }

    pub fn id(&self) -> &CoupleID {
        &self.id
    }
}


/// Unique identifier for a couple, internal use only.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoupleID(i64);

impl From<i64> for CoupleID {
    fn from(raw: i64) -> Self {
        Self(raw)
    }
}

impl Into<i64> for CoupleID {
    fn into(self) -> i64 {
        self.0
    }
}


// --- questions ---

/// A single prompt template. Note that `response_type` is for deciding
/// what type of data the corresponding `Answer`s should store/for
/// giving a user the correct way to answer a given question. See
/// `ResponseType` for more details.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Question {
    id: QuestionID,
    category: QuestionCategory,
    prompt: String,
    response_type: ResponseType,
}


/// Unique identifier for a question, internal only.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QuestionID(i64);


/// A label for grouping questions/what type of topic.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuestionCategory {}


/// How a user can respond/what type of input to provide. Note an instance
/// of `ResponseType` *only* sets the boundaries of how to respond (e.g.
/// a user should respond with a number between 1 and 5, or free-form
/// input to type anything they want). In other words, this is intended as
/// a marker, not as a content store.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResponseType {
    Text,
    YesNo,
    Number(Range<usize>),
    MultipleChoice(Vec<String>),
}


// --- answers ---

/// A single response to a specified question, i.e. this **user**'s
/// response to this question, not both users/partners to the question.
/// Note that `response` is the actual content of the reply/the data,
/// matched to what type of response is specified in `response_type` of
/// the `Question`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Answer {
    id: AnswerID,
    question_id: QuestionID,
    user_id: UserID,
    timestamp: Zoned,
    response: Response,
}


/// Unique identifier for an answer, internal only.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnswerID(i64);


// TODO: what to do about this duplication/will it become complicated to
// handle?
/// What the user responded to a question with/the actual input provided.
/// 1-to-1 with `ResponseType`'s options.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Response {
    Text(String),
    YesNo(bool),
    Number(usize),
    MultipleChoice(usize),
}


// --- locations ---

/// A single location datapoint (of a specific user at a specified instant
/// in time).
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Location {
    id: LocationID,
    user_id: UserID,
    timestamp: Zoned,
    latitude: Latitude,
    longitude: Longitude,
    accuracy: u32,
}


/// Unique identifier for a location, internal only.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocationID(i64);


/// North-south component of location datapoint.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Latitude(f64);


/// West-east component of location datapoint.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Longitude(f64);


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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Heartbeat {
    id: HeartbeatID,
    user_id: UserID,
    start_timestamp: Zoned,
    end_timestamp: Option<Zoned>,
}


/// Unique identifier for a heartbeat, internal only.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeartbeatID(i64);
