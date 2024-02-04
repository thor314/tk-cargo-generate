use anyhow::{anyhow, Result};
use log::trace;

use crate::error::MyError;
{% if cli -%}
use clap::Parser;
use crate::cli::MyArgs;
// or:
// use crate::{cli::subcommand::SubcommandArgs as MyArgs, error::MyError};

/// Set up crate logging and environment variables.
pub(crate) fn setup() -> Result<MyArgs, MyError> {
  dotenv::dotenv().ok();
  // init_tracing();
  let args = MyArgs::parse();
  if std::env::var("DOTENV_OK").is_ok() {
    trace!("loaded dotenv");
  } else {
    return Err(anyhow!("failed to load dotenv").into());
  }
  env_logger::builder().filter_level(args.log_level()).build();

  Ok(args)
}

{% else -%}
/// Set up crate logging and environment variables.
pub(crate) fn setup() -> Result<(), MyError> {
  dotenv::dotenv().ok();
  {% if async -%} init_tracing();  {% else -%}
  env_logger::init();
  {% endif -%}
  if std::env::var("DOTENV_OK").is_ok() {
    trace!("loaded dotenv");
  } else {
    return Err(anyhow!("failed to load dotenv").into());
  }

  Ok(())
}
{% endif -%}

{% if async -%}
// todo: doesn't feature onto what we've done with clap args yet
/// Set up the tracing filter level using the env value, or else set it here. Reads RUST_LOG.
/// TRACE < DEBUG < INFO < WARN < ERROR
#[tracing::instrument]
pub(crate) fn init_tracing() {
  // todo: env-logger
  let filter = tracing::level_filters::LevelFilter::INFO.into();
  // set level to RUST_LOG env variable, or else INFO
  let filter =
    tracing_subscriber::EnvFilter::builder().with_default_directive(filter).from_env_lossy();
  //  .with_level(false) // don't include levels in formatted output
  //  .with_target(false) // don't include targets
  //  .with_thread_ids(true) // include the thread ID of the current thread
  //  .with_thread_names(true) // include the name of the current thread
  //  .compact(); // use the `Compact` formatting style.
  tracing_subscriber::fmt().with_env_filter(filter).init();
}
{% endif -%}
