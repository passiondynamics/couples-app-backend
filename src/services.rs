//! Author: irith
//! Date: 2025-11-03 @ 6:47pm
//! Description: TODO

use tracing_subscriber::{
    self,
    EnvFilter,
};


pub struct LogService {}

impl LogService {
    pub fn init() {
        tracing_subscriber::fmt()
                           .with_level(true)
                           .with_target(true)
                           .with_env_filter(EnvFilter::from_default_env())
                           .init();
    }
}
