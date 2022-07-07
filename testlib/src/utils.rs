//! Utils for the crate
//! https://github.com/Keats/validator

use anyhow::Result;
use crate::error::MyError;


/// Set up the log level using the env value, or else set it here.
pub(crate) fn init_logger() {
	let level = "info";
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(level)).init();
}
