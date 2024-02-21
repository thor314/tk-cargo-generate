use anyhow::{anyhow, Context};
{% if async -%} 
use tracing_subscriber::{filter::{EnvFilter, LevelFilter}, layer::SubscriberExt, util::SubscriberInitExt};
use tracing::trace;
{% else -%} 
use log::trace;
{% endif -%}
use crate::error::MyError;
{% if cli -%}
use clap::Parser;
use crate::cli::MyArgs;
// use crate::{cli::subcommand::SubcommandArgs as MyArgs, error::MyError};
{% endif -%}

/// Set up crate logging and environment variables.
{% if cli -%}
  pub(crate) fn setup(
    {% if server -%} secret_store: &shuttle_secrets::SecretStore {% endif -%}
  ) -> Result<MyArgs, MyError> {
    {% if server -%} {% else %} dotenvy::dotenv().ok(); {% endif -%}
    let args = MyArgs::parse();
    {% if async -%}
      let filter = EnvFilter::builder()
        .with_default_directive(args.log_level().into())
        .from_env_lossy();
      tracing_subscriber::fmt().with_env_filter(filter).init();
    {% else -%}
      env_logger::builder().filter_level(args.log_level()).build();
    {% endif -%}
{% else -%}
  pub(crate) fn setup(
    {% if server -%} secret_store: &shuttle_secrets::SecretStore {% endif -%}
      ) -> Result<(), MyError> {
    {% if server -%} {% else %} dotenvy::dotenv().ok(); {% endif -%}
    {% if async -%} 
      let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
      tracing_subscriber::fmt().with_env_filter(filter).init();
    {% else -%}
      env_logger::init();
    {% endif -%}
{% endif -%}

  {% if server -%} secret_store.get("DOTENV_OK").context("failed to get secrets")?; {% else -%} 
  std::env::var("DOTENV_OK").with_context(|| anyhow!("failed to load dotenv"))?; {% endif -%}

  {% if cli %} Ok(args) {% else %} Ok(()) {% endif %}
}
