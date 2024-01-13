use anyhow::{anyhow, Result};
use log::{error, info};

use crate::error::MyError;

/// Set up crate logging and environment variables.
pub(crate) fn setup() -> Result<(), MyError> {
  dotenv::dotenv().ok();
  env_logger::init();
  // tracing::init_tracing(); // async alternative
  if std::env::var("DOTENV_OK").is_ok() {
    info!("loaded dotenv");
  } else {
    error!("failed to load dotenv");
    return Err(anyhow!("failed to load dotenv").into());
  }

  // let args = TkCLI::parse();
  // Ok(args)
  Ok(())
}

