//! Author: irith
//! Date: 2025-11-03 @ 10:43pm
//! Description: Define what (backend) behavior is configurable, via
//!              environment variables.

use std::env;
use std::path::PathBuf;

use crate::constants::{
    DEFAULT_APPDATA_DIR,
    DEFAULT_PORT,
};


/// Stores the environment variable config at startup.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Config {
    pub appdata_dir: PathBuf,
    pub port: u16,
}

impl Config {
    /// Read the corresponding environment variables, defaulting if not
    /// present.
    pub fn from_env() -> Self {
        let appdata_dir = PathBuf::from(env::var("APPDATA_DIR")
                                            .unwrap_or(DEFAULT_APPDATA_DIR.to_owned()));
        let port = env::var("PORT")
                       .ok()
                       .and_then(|p| p.parse::<u16>().ok())
                       .unwrap_or(DEFAULT_PORT);

        Self {appdata_dir, port}
    }
}
