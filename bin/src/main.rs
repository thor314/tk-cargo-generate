#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use error::MyError;
use tracing::{debug, info};

mod error;
#[cfg(test)] mod tests;
mod utils;

fn main() -> Result<(), MyError> {
  utils::setup()?;
  if std::env::var("DOTENV_OK").is_ok() {
    info!("Info :D");
  }
  Ok(())
}
