//! Crate error types with thiserror
//! https://docs.rs/thiserror/latest/thiserror/

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
	#[error("My Io error")]
	Io(#[from] std::io::Error),
	#[error("an unhandled error")]
	Unhandled,
}
