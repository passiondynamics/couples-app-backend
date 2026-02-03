//! Author: irith
//! Date: 2025-11-03 @ 6:47pm
//! Description: Translation layer for data storage from intent (upstairs
//! from a given service) to data change actions/details of making it
//! happen (downstairs in the database).

use anyhow::{
    Context,
    Result,
};
use bincode::encode_to_vec;
use sqlx::query;
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
use tracing::debug;
use tracing::info;

use std::fs;
use std::mem::swap;
use std::str::FromStr;

use crate::constants::{
    BINCODE_CONFIG,
    DB_FILENAME,
    MIGRATIONS_DIR,
};
use crate::models::config::Config;
use crate::models::data::{
    Answer,
    Heartbeat,
    Location,
    Couple,
    Question,
    User,
    UserPreferences,
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
    RemoveQuestionError,
    RemoveQuestionRequest,
    RemoveUserError,
    RemoveUserRequest,
    SetCoupleError,
    SetCoupleRequest,
    StartHeartbeatError,
    StartHeartbeatRequest,
    UnsetCoupleError,
    UnsetCoupleRequest,
};


/// A data storage access layer for the app that can be sent over threads.
pub trait DatabaseInterface: Clone + Send + Sync + 'static {
    /// Create a new user in the database.
    fn add_user(&self, request: &AddUserRequest) -> impl Future<Output = Result<User, AddUserError>> + Send;

    /// Remove a given user from the database.
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

    // TODO: any reason to have `remove_answer`

    /// Add a new location datapoint.
    fn add_location(&self, request: &AddLocationRequest) -> impl Future<Output = Result<Location, AddLocationError>> + Send;

    // TODO: any reason to have individual
    // `remove_location`/`remove_heartbeat` instead of on bulk by
    // `*_history_preference`?

    /// Add a new heartbeat range.
    fn start_heartbeat(&self, request: &StartHeartbeatRequest) -> impl Future<Output = Result<Heartbeat, StartHeartbeatError>> + Send;

    /// Mark a given heartbeat range as finished.
    fn end_heartbeat(&self, request: &EndHeartbeatRequest) -> impl Future<Output = Result<Heartbeat, EndHeartbeatError>> + Send;
}


/// An implementation of (outgoing) data storage access to a SQLite
/// database to dictate data change actions.
#[derive(Debug, Clone)]
pub struct SQLiteInterface {
    pool: SqlitePool,
}

impl SQLiteInterface {
    /// Create a new database if not already present, set up a connection
    /// pool, and update the schema if needed.
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
        sqlite.validate_schema(config)
              .await
              .with_context(|| "could not validate schema")?;

        Ok(sqlite)
    }

    /// Use the migration SQL files to ensure the database has the correct
    /// schema before use.
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
            .with_context(|| "could not run migrator")?;

        Ok(())
    }
}

impl DatabaseInterface for SQLiteInterface {
    async fn add_user(&self, request: &AddUserRequest) -> Result<User, AddUserError> {
        // Use request + defaults from `UserPreferences` to build an
        // insert statement.
        let username = request.username().clone();
        let password = request.password().clone();
        let preferences = UserPreferences::new();
        let query = query(r#"
                INSERT INTO user
                    (username, password, new_feature_notifications, location_history, heartbeat_history)
                VALUES
                    ($1, $2, $3, $4, $5)
            "#)
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
                          // If our `username` constraint is violated.
                          Some(err) if err.is_unique_violation() => AddUserError::UsernameExists(username.as_str().to_string()),

                          // Otherwise it's an issue with the statement
                          // itself.
                          _ => AddUserError::InsertFailure(e.to_string()),
                      })?
                      .last_insert_rowid();

        // Return a corresponding user object.
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


    async fn remove_user(&self, request: &RemoveUserRequest) -> Result<(), RemoveUserError> {
        // Use given user ID/primary key to build a delete statement.
        let query = query(r#"
                DELETE FROM user
                WHERE id = $1
            "#)
            .bind(request.user_id());

        let count = query.execute(&self.pool)
                         .await
                         .map_err(|e| RemoveUserError::DeleteFailure(e.to_string()))?
                         .rows_affected();

        // Verify that we actually removed exactly one user.
        if count == 1 {
            Ok(())
        } else {
            Err(RemoveUserError::UserNotFound)
        }
    }

    async fn set_couple(&self, request: &SetCoupleRequest) -> Result<Couple, SetCoupleError> {
        let mut user_id_1 = request.user_id_1();
        let mut user_id_2 = request.user_id_2();

        // Keep the IDs in ascending order.
        if user_id_1 > user_id_2 {
            swap(&mut user_id_1, &mut user_id_2);
        };

        let query = query(r#"
                INSERT INTO couple
                    (user_id_1, user_id_2)
                VALUES
                    ($1, $2)
            "#)
            .bind(user_id_1)
            .bind(user_id_2);

        let id = query.execute(&self.pool)
                      .await
                      .map_err(|e| match e.as_database_error() {
                          // Either the `user_id_1` or `user_id_2` foreign
                          // key constraint failed.
                          Some(err) if err.is_foreign_key_violation() => SetCoupleError::UserNotFound,
                          _ => SetCoupleError::InsertFailure(e.to_string()),
                      })?
                      .last_insert_rowid();

        Ok(Couple::new(
            id,
            user_id_1,
            user_id_2,
        ))
    }

    async fn unset_couple(&self, request: &UnsetCoupleRequest) -> Result<(), UnsetCoupleError> {
        let query = query(r#"
                DELETE FROM couple
                WHERE id = $1
            "#)
            .bind(request.couple_id());

        let count = query.execute(&self.pool)
                         .await
                         .map_err(|e| UnsetCoupleError::DeleteFailure(e.to_string()))?
                         .rows_affected();

        if count == 1 {
            Ok(())
        } else {
            Err(UnsetCoupleError::CoupleNotFound)
        }
    }

    async fn add_question(&self, request: &AddQuestionRequest) -> Result<Question, AddQuestionError> {
        // Enums are encoded via `bincode` and inserted as a `BLOB` type.
        let category = request.category();
        let category_enc = encode_to_vec(category, BINCODE_CONFIG)
                           .map_err(|e| AddQuestionError::EncodeFailure(e.to_string()))?;
        let prompt = request.prompt().clone();
        let answer_type = request.answer_type().clone();
        let answer_type_enc = encode_to_vec(&answer_type, BINCODE_CONFIG)
                              .map_err(|e| AddQuestionError::EncodeFailure(e.to_string()))?;

        let query = query(r#"
                INSERT INTO question
                    (category, prompt, answer_type)
                VALUES
                    ($1, $2, $3)
            "#)
            .bind(category_enc)
            .bind(&prompt)
            .bind(answer_type_enc);

        let id = query.execute(&self.pool)
                      .await
                      .map_err(|e| AddQuestionError::InsertFailure(e.to_string()))?
                      .last_insert_rowid();

        Ok(Question::new(
            id,
            category,
            prompt,
            answer_type,
        ))
    }

    async fn remove_question(&self, request: &RemoveQuestionRequest) -> Result<(), RemoveQuestionError> {
        let query = query(r#"
                DELETE FROM question
                WHERE id = $1
            "#)
            .bind(request.question_id());

        let count = query.execute(&self.pool)
                         .await
                         .map_err(|e| RemoveQuestionError::DeleteFailure(e.to_string()))?
                         .rows_affected();

        if count == 1 {
            Ok(())
        } else {
            Err(RemoveQuestionError::QuestionNotFound)
        }
    }

    async fn add_answer(&self, request: &AddAnswerRequest) -> Result<Answer, AddAnswerError> {
        let question_id = request.question_id();
        let user_id = request.user_id();
        let timestamp = request.timestamp().clone();
        let timestamp_enc = timestamp.to_string();
        let content = request.content().clone();
        let content_enc = encode_to_vec(&content, BINCODE_CONFIG)
                          .map_err(|e| AddAnswerError::EncodeFailure(e.to_string()))?;

        let query = query(r#"
                INSERT INTO answer
                    (question_id, user_id, timestamp, content)
                VALUES
                    ($1, $2, $3, $4)
            "#)
            .bind(question_id)
            .bind(user_id)
            .bind(timestamp_enc)
            .bind(content_enc);

        let id = query.execute(&self.pool)
                      .await
                      .map_err(|e| match e.as_database_error() {
                          // Unfortunately, we can't tell *which*
                          // constraint fails (limitation of SQLite3).
                          Some(err) if err.is_foreign_key_violation() => AddAnswerError::QuestionOrUserNotFound,
                          _ => AddAnswerError::InsertFailure(e.to_string()),
                      })?
                      .last_insert_rowid();

        Ok(Answer::new(
            id,
            question_id,
            user_id,
            timestamp,
            content,
        ))
    }

    async fn add_location(&self, request: &AddLocationRequest) -> Result<Location, AddLocationError> {
        let user_id = request.user_id();
        let timestamp = request.timestamp().clone();
        let timestamp_enc = timestamp.to_string();

        // We unwrap both of these newtypes before binding them.
        let latitude = request.latitude();
        let latitude_enc: f64 = latitude.into();
        let longitude = request.longitude();
        let longitude_enc: f64 = longitude.into();
        let accuracy = request.accuracy();

        let query = query(r#"
                INSERT INTO location
                    (user_id, timestamp, latitude, longitude, accuracy)
                VALUES
                    ($1, $2, $3, $4, $5)
            "#)
            .bind(user_id)
            .bind(timestamp_enc)
            .bind(latitude_enc)
            .bind(longitude_enc)
            .bind(accuracy);

        let id = query.execute(&self.pool)
                      .await
                      .map_err(|e| match e.as_database_error() {
                          Some(err) if err.is_foreign_key_violation() => AddLocationError::UserNotFound,
                          _ => AddLocationError::InsertFailure(e.to_string()),
                      })?
                      .last_insert_rowid();

        Ok(Location::new(
            id,
            user_id,
            timestamp,
            latitude,
            longitude,
            accuracy,
        ))
    }

    async fn start_heartbeat(&self, request: &StartHeartbeatRequest) -> Result<Heartbeat, StartHeartbeatError> {
        let user_id = request.user_id();
        let start_timestamp = request.start_timestamp().clone();
        let start_timestamp_enc = start_timestamp.to_string();

        let query = query(r#"
                INSERT INTO heartbeat
                    (user_id, start_timestamp)
                VALUES
                    ($1, $2)
            "#)
            .bind(user_id)
            .bind(start_timestamp_enc);

        let id = query.execute(&self.pool)
                      .await
                      .map_err(|e| match e.as_database_error() {
                          Some(err) if err.is_foreign_key_violation() => StartHeartbeatError::UserNotFound,
                          _ => StartHeartbeatError::InsertFailure(e.to_string()),
                      })?
                      .last_insert_rowid();

        let end_timestamp = None;
        Ok(Heartbeat::new(
            id,
            user_id,
            start_timestamp,
            end_timestamp,
        ))
    }

    async fn end_heartbeat(&self, request: &EndHeartbeatRequest) -> Result<Heartbeat, EndHeartbeatError> {
        todo!()
    }
}
