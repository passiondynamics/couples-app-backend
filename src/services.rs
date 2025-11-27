//! Author: irith
//! Date: 2025-11-03 @ 6:47pm
//! Description: TODO

use tracing_subscriber::{
    self,
    EnvFilter,
};

use crate::interfaces::database::DatabaseInterface;
use crate::models::data::{
    Answer,
    Heartbeat,
    Location,
    Couple,
    Question,
    User,
};
use crate::models::interface::{
    AddAnswerError,
    AddAnswerRequest,
    AddLocationError,
    AddLocationRequest,
    AddQuestionError,
    AddQuestionRequest,
    AddUserError,
    AddUserRequest,
    EndHeartbeatError,
    EndHeartbeatRequest,
    RemoveQuestionRequest,
    RemoveQuestionError,
    RemoveUserRequest,
    RemoveUserError,
    SetCoupleError,
    SetCoupleRequest,
    StartHeartbeatError,
    StartHeartbeatRequest,
    UnsetCoupleRequest,
    UnsetCoupleError,
};


/// A shim service to abstract logging initialization for the rest of the
/// app to use.
#[derive(Debug, Clone)]
pub struct LogService {}

impl LogService {
    /// Set up logging for the app.
    pub fn init() {
        tracing_subscriber::fmt()
                           .with_level(true)
                           .with_target(true)
                           .with_env_filter(EnvFilter::from_default_env())
                           .init();
    }
}


/// We call a "application service" anything that implements the core logic of
/// the application (what does our app actually do). Other ways it could
/// be referred to are:
///   - the "domain" (yet another jargon term that doesn't give us
///     intuition for what it does),
///   - implements/contains the "business logic" (ew),
///   - a "core service" (better but still a jargon-y/loaded term).
///
/// At any given point, there's probably:
///   - only one implementation written (95% likely),
///   - more than one written, but only one running/actually doing stuff
///     while writing or transitioning to another (5%? 1%? if ever? pick a
///     small number).
///
/// Note this is (currently) very similar to the `DatabaseInterface`'s
/// definition. In getting basic functionality working, we're starting
/// with 1-1 for what the intent given to the backend is to how we're
/// storing the data, but as the application logic becomes more
/// complicated, we want the modularity down the line to have intents
/// coordinate other things/do different things than what we do with data
/// storage (thus why they're two distinct traits).
pub trait AppService: Clone + Send + Sync + 'static {
    /// Create a new user.
    fn add_user(&self, request: &AddUserRequest) -> impl Future<Output = Result<User, AddUserError>> + Send;

    /// Remove a given user.
    fn remove_user(&self, request: &RemoveUserRequest) -> impl Future<Output = Result<(), RemoveUserError>> + Send;

    /// Associate two users together as a couple.
    fn set_couple(&self, request: &SetCoupleRequest) -> impl Future<Output = Result<Couple, SetCoupleError>> + Send;

    /// Disassociate the given couple from each other.
    fn unset_couple(&self, request: &UnsetCoupleRequest) -> impl Future<Output = Result<(), UnsetCoupleError>> + Send;

    /// Add a new question.
    fn add_question(&self, request: &AddQuestionRequest) -> impl Future<Output = Result<Question, AddQuestionError>> + Send;

    /// Remove a given question.
    fn remove_question(&self, request: &RemoveQuestionRequest) -> impl Future<Output = Result<(), RemoveQuestionError>> + Send;

    /// Add a new answer.
    fn add_answer(&self, request: &AddAnswerRequest) -> impl Future<Output = Result<Answer, AddAnswerError>> + Send;

    /// Add a new location datapoint.
    fn add_location(&self, request: &AddLocationRequest) -> impl Future<Output = Result<Location, AddLocationError>> + Send;

    /// Add a new heartbeat range.
    fn start_heartbeat(&self, request: &StartHeartbeatRequest) -> impl Future<Output = Result<Heartbeat, StartHeartbeatError>> + Send;

    /// Mark a given heartbeat range as finished.
    fn end_heartbeat(&self, request: &EndHeartbeatRequest) -> impl Future<Output = Result<Heartbeat, EndHeartbeatError>> + Send;
}


/// An implementation of the application logic. Again, probably the only
/// one, unless we choose to rewrite it down the line (not likely). Takes
/// any data storage interface (generic over `DatabaseInterface`).
#[derive(Debug, Clone)]
pub struct CouplesAppService<D>
where
    D: DatabaseInterface,
{
    database: D,
}

impl<D> CouplesAppService<D>
where
    D: DatabaseInterface,
{
    /// Create a new service that uses the given data storage interface.
    pub fn new(database: D) -> Self {
        Self {database}
    }
}

impl<D> AppService for CouplesAppService<D>
where
    D: DatabaseInterface,
{
    async fn add_user(&self, request: &AddUserRequest) -> Result<User, AddUserError> {
        self.database.add_user(request).await
    }

    async fn remove_user(&self, request: &RemoveUserRequest) -> Result<(), RemoveUserError> {
        self.database.remove_user(request).await
    }

    async fn set_couple(&self, request: &SetCoupleRequest) -> Result<Couple, SetCoupleError> {
        self.database.set_couple(request).await
    }

    async fn unset_couple(&self, request: &UnsetCoupleRequest) -> Result<(), UnsetCoupleError> {
        self.database.unset_couple(request).await
    }

    async fn add_question(&self, request: &AddQuestionRequest) -> Result<Question, AddQuestionError> {
        self.database.add_question(request).await
    }

    async fn remove_question(&self, request: &RemoveQuestionRequest) -> Result<(), RemoveQuestionError> {
        self.database.remove_question(request).await
    }

    async fn add_answer(&self, request: &AddAnswerRequest) -> Result<Answer, AddAnswerError> {
        self.database.add_answer(request).await
    }

    async fn add_location(&self, request: &AddLocationRequest) -> Result<Location, AddLocationError> {
        self.database.add_location(request).await
    }

    async fn start_heartbeat(&self, request: &StartHeartbeatRequest) -> Result<Heartbeat, StartHeartbeatError> {
        self.database.start_heartbeat(request).await
    }

    async fn end_heartbeat(&self, request: &EndHeartbeatRequest) -> Result<Heartbeat, EndHeartbeatError> {
        self.database.end_heartbeat(request).await
    }
}
