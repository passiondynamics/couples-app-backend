//! Author: irith
//! Date: 2025-11-03 @ 6:47pm
//! Description: TODO

use anyhow::{
    Context,
    Result,
};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{
    Sqlite,
    SqliteConnectOptions,
    SqliteJournalMode,
    SqlitePool,
};
use tracing::{
    debug,
    info,
};

use std::path::PathBuf;
use std::str::FromStr;

use crate::constants::DB_FILENAME;
use crate::models::config::Config;
use crate::models::data::{
    Answer,
    Heartbeat,
    Location,
    Partner,
    PartnerID,
    Question,
    QuestionID,
    User,
    UserID,
};
use crate::models::interface::{
    AddAnswerError,
    AddAnswerRequest,
    AddLocationError,
    AddLocationRequest,
    AddPartnerError,
    AddPartnerRequest,
    AddQuestionError,
    AddQuestionRequest,
    AddUserError,
    AddUserRequest,
    EndHeartbeatError,
    EndHeartbeatRequest,
    RemovePartnerError,
    RemoveQuestionError,
    RemoveUserError,
    StartHeartbeatError,
    StartHeartbeatRequest,
};


pub trait DatabaseInterface: Clone + Send + Sync + 'static {
    fn add_user(&self, request: &AddUserRequest) -> impl Future<Output = Result<User, AddUserError>> + Send;

    fn remove_user(&self, user_id: UserID) -> impl Future<Output = Result<User, RemoveUserError>> + Send; // TODO: removes should return `()`?

    fn add_partner(&self, request: &AddPartnerRequest) -> impl Future<Output = Result<Partner, AddPartnerError>> + Send;

    fn remove_partner(&self, partner_id: PartnerID) -> impl Future<Output = Result<Partner, RemovePartnerError>> + Send;

    fn add_question(&self, request: &AddQuestionRequest) -> impl Future<Output = Result<Question, AddQuestionError>> + Send;

    fn remove_question(&self, question_id: QuestionID) -> impl Future<Output = Result<Question, RemoveQuestionError>> + Send;

    fn add_answer(&self, request: &AddAnswerRequest) -> impl Future<Output = Result<Answer, AddAnswerError>> + Send;

    // TODO: any reason to have `remove_answer`

    fn add_location(&self, request: &AddLocationRequest) -> impl Future<Output = Result<Location, AddLocationError>> + Send;

    // TODO: any reason to have individual
    // `remove_location`/`remove_heartbeat` instead of on bulk by
    // `*_history_preference`?

    fn start_heartbeat(&self, request: &StartHeartbeatRequest) -> impl Future<Output = Result<Heartbeat, StartHeartbeatError>> + Send;

    fn end_heartbeat(&self, request: &EndHeartbeatRequest) -> impl Future<Output = Result<Heartbeat, EndHeartbeatError>> + Send;
}


#[derive(Debug, Clone)]
pub struct SqliteInterface {
    pool: SqlitePool,
}

impl SqliteInterface {
    pub async fn new(config: &Config) -> Result<Self> {
        let path = config.appdata_dir.join(DB_FILENAME);
        let url = format!("sqlite://{}", path.display());
        let db_exists = Sqlite::database_exists(&url)
                               .await
                               .unwrap_or(false);
        if !db_exists {
            Sqlite::create_database(&url)
                   .await
                   .with_context(|| format!("could not create sqlite database @ `{}`", url))?;
            info!("Created sqlite database @ `{}`", url);
        }

        let options = SqliteConnectOptions::from_str(&url)?
                                           .journal_mode(SqliteJournalMode::Wal);
        let pool = SqlitePool::connect_with(options)
                              .await
                              .with_context(|| format!("could not open sqlite database @ `{}`", url))?;
        info!("Opened sqlite database @ `{}`", url);

        Ok(Self {pool})
    }
}

impl DatabaseInterface for SqliteInterface {
    async fn add_user(&self, request: &AddUserRequest) -> Result<User, AddUserError> {
        todo!()
    }

    async fn remove_user(&self, user_id: UserID) -> Result<User, RemoveUserError> {
        todo!()
    }

    async fn add_partner(&self, request: &AddPartnerRequest) -> Result<Partner, AddPartnerError> {
        todo!()
    }

    async fn remove_partner(&self, partner_id: PartnerID) -> Result<Partner, RemovePartnerError> {
        todo!()
    }

    async fn add_question(&self, request: &AddQuestionRequest) -> Result<Question, AddQuestionError> {
        todo!()
    }

    async fn remove_question(&self, question_id: QuestionID) -> Result<Question, RemoveQuestionError> {
        todo!()
    }

    async fn add_answer(&self, request: &AddAnswerRequest) -> Result<Answer, AddAnswerError> {
        todo!()
    }

    async fn add_location(&self, request: &AddLocationRequest) -> Result<Location, AddLocationError> {
        todo!()
    }

    async fn start_heartbeat(&self, request: &StartHeartbeatRequest) -> Result<Heartbeat, StartHeartbeatError> {
        todo!()
    }

    async fn end_heartbeat(&self, request: &EndHeartbeatRequest) -> Result<Heartbeat, EndHeartbeatError> {
        todo!()
    }
}
