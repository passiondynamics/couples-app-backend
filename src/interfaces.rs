//! Author: irith
//! Date: 2025-11-03 @ 6:47pm
//! Description: Translation layers from intent (upstairs from a given
//!              service) to details of making it happen (downstairs in
//!              the data storage, or with an API, or just *somewhere
//!              else*).

use anyhow::{
    Context,
    Result,
};
use sqlx::{
    query,
    Executor,
};
use sqlx::migrate::{
    MigrateDatabase,
    Migrator,
};
use sqlx::sqlite::{
    Sqlite,
    SqliteConnectOptions,
    SqliteJournalMode,
    SqlitePool,
};
use tracing::{
    debug,
    error,
    info,
};

use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use crate::constants::{
    DB_FILENAME,
    MIGRATIONS_DIR,
};
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
    UserPreferences,
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

        let sqlite = Self {pool};
        sqlite.validate_schema(config).await.with_context(|| "could not validate schema")?;

        Ok(sqlite)
    }

    async fn validate_schema(&self, config: &Config) -> Result<()> {
        let path = config.appdata_dir.join(MIGRATIONS_DIR);
        let dir_exists = fs::metadata(&path)
                            .map_or(false, |d| d.is_dir());
        if !dir_exists {
            fs::create_dir(&path)?;
        }

        Migrator::new(path)
            .await
            .with_context(|| "could not create migrator")?
            .run(&self.pool)
            .await
            .with_context(|| "could not run migrator")
    }
}

impl DatabaseInterface for SqliteInterface {
    async fn add_user(&self, request: &AddUserRequest) -> Result<User, AddUserError> {
        // Use request + defaults from `UserPreferences` to build an
        // insert query.
        let username = request.username().clone();
        let password = request.password().clone();
        let preferences = UserPreferences::new();
        let query = query(r#"
                INSERT INTO user
                    (username, password, new_feature_notifications, location_history, heartbeat_history)
                VALUES
                    ($1, $2, $3, $4, $5)
                RETURNING
                    id"#,
            )
            .bind(username.as_str())
            .bind(password.as_str())
            .bind(preferences.new_feature_notifications())
            .bind(preferences.location_history())
            .bind(preferences.heartbeat_history());

        // Use the corresponding `rowid` (since it's our primary key) as
        // our user's ID.
        let id = query.execute(&self.pool)
                      .await
                      .map_err(|e| match e.as_database_error() {
                          Some(e) if e.is_unique_violation() => AddUserError::UsernameExists(username.as_str().to_string()),
                          _ => AddUserError::Unknown(e.to_string()),
                      })?
                      .last_insert_rowid();

        Ok(User::new(
            id,
            username,
            password,
            preferences,
        ))
    }
    // let mut tx = self.pool.begin()
    //                       .await
    //                       .map_err(|_| AddUserError::TransactionStart)?;
    // let id = tx.execute(query)
    //            .await
    //            .inspect_err(|e| error!("Could not execute transaction: {}", e))
    //            .map_err(|_| AddUserError::TransactionExecute)?
    //            .last_insert_rowid();
    // tx.commit()
    //   .await
    //   .map_err(|_| AddUserError::TransactionCommit)?;


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
