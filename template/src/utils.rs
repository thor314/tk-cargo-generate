use anyhow::{anyhow, Result};
{% if async -%} 
use tracing_subscriber::{filter::{EnvFilter, LevelFilter}, layer::SubscriberExt, util::SubscriberInitExt};
use tracing::trace;
{% else -%} log::trace;
{% endif %}

use crate::error::MyError;
{% if cli -%}
use clap::Parser;
use crate::cli::MyArgs;
// use crate::{cli::subcommand::SubcommandArgs as MyArgs, error::MyError};
{% endif -%}


/// Set up crate logging and environment variables.
// #[tracing::instrument]
{% if cli -%}
  pub(crate) fn setup() -> Result<MyArgs, MyError> {
    dotenv::dotenv().ok();
    let args = MyArgs::parse();
    {% if async -%}
      let filter = EnvFilter::builder()
        .with_default_directive(args.log_level().into())
        .from_env_lossy();
      tracing_subscriber::registry().with(filter).init();
    {% else -%}
      env_logger::builder().filter_level(args.log_level()).build();
    {% endif -%}
{% else -%}
  pub(crate) fn setup() -> Result<(), MyError> {
    dotenv::dotenv().ok();
    {% if async -%} 
      let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
      tracing_subscriber::registry().with(filter).init();
    {% else -%}
      env_logger::init();
    {% endif -%}
{% endif %}

  if std::env::var("DOTENV_OK").is_ok() {
    trace!("loaded dotenv");
  } else {
    return Err(anyhow!("failed to load dotenv").into());
  }

  {% if cli %} Ok(args) {% else %} Ok(()) {% endif %}
}
