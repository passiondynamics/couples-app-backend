//! Author: irith
//! Date: 2025-11-03 @ 10:47pm
//! Description: Static constants to be used.

use bincode::config;
use regex::Regex;

use std::sync::OnceLock;


pub const DEFAULT_APPDATA_DIR: &'static str = "/etc/couples-app-backend/";
pub const DEFAULT_PORT: u16 = 10000;

pub const DB_FILENAME: &'static str = "data.db";
pub const MIGRATIONS_DIR: &'static str = "migrations";

/// A Regex constant for an alphanumeric username with 4 or more
/// characters.
pub fn get_username_regex() -> &'static Regex {
    static USERNAME_REGEX: OnceLock<Regex> = OnceLock::new();
    USERNAME_REGEX.get_or_init(|| Regex::new(r"[A-Za-z0-9-_\.]{4,}").expect("Could not create username regex"))
}

pub const MIN_PASSWORD_LEN: usize = 12;

pub const BINCODE_CONFIG: config::Configuration = config::standard();
