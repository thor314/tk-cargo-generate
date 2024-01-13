use anyhow::{anyhow, Result};
{% if cli %}
use clap::Parser;
pub use cli::Cli;
{%- endif %}
use log::{error, info};

use crate::error::MyError;

/// Set up crate logging and environment variables.
{% if cli %}
pub(crate) fn setup() -> Result<Cli, MyError> {
{% else %}
pub(crate) fn setup() -> Result<(), MyError> {
{%- endif %}
  dotenv::dotenv().ok();
  env_logger::init();
  // tracing_init::init_tracing(); // async alternative
  if std::env::var("DOTENV_OK").is_ok() {
    info!("loaded dotenv");
  } else {
    error!("failed to load dotenv");
    return Err(anyhow!("failed to load dotenv").into());
  }

  {% if cli %}
  let args = cli::Cli::parse();
  Ok(args)
  {%- endif %}
  Ok(())
}

mod tracing_init {
  /// Set up the tracing filter level using the env value, or else set it here. Reads RUST_LOG.
  /// TRACE < DEBUG < INFO < WARN < ERROR
  #[tracing::instrument]
  pub(crate) fn init_tracing() {
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
}

mod cli {
  use clap::{Args, Parser};

  // https://docs.rs/clap/latest/clap/
  /// CLI parser
  #[derive(Parser, Debug)]
  #[command(name = "tk")]
  #[command(bin_name = "tk")]
  pub enum Cli {
    Testing(TestArgs),
  }

  /// Simple program to greet a person
  #[derive(Debug, Args)]
  #[command(author, version, about, long_about = None)]
  pub struct TestArgs {
    /// Name of the person to greet
    #[arg(short, long)]
    pub name: String,
    // /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub count: usize,
  }
}
