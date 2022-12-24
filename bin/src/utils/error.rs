//! Crate error types with thiserror
//! https://docs.rs/thiserror/latest/thiserror/

use thiserror::Error;

// https://docs.rs/thiserror/latest/thiserror/
#[derive(Debug, Error)]
pub enum MyError {
  #[error("My Io error: {0}")]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Anyhow(#[from] anyhow::Error),
  #[allow(dead_code)]
  #[error("an unhandled error")]
  Unhandled,
}
