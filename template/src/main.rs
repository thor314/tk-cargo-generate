#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use error::MyError;
use log::{info, error};

// mod error;
// #[cfg(test)] mod tests;

fn main() -> Result<(), MyError> {
  if std::env::var("DOTENV_OK").is_ok() {
    info!("loaded dotenv");
  } else {
    error!("failed to load dotenv")
  }

  Ok(())
}
