use clap::{ArgAction, Args, Parser};
use log::LevelFilter;

#[derive(Parser, Debug)]
#[command(name = "tk")]
#[command(bin_name = "tk")]
#[clap(about = "tk does things")]
#[command(author, version, long_about = None)]
pub struct MyArgs {
  /// Set the verbosity. Use -v for DEBUG, -vv for TRACE. None for INFO.
  #[arg(long = "verbose", short = 'v', action = ArgAction::Count)]
  pub verbosity: u8,
}

impl MyArgs {
  /// in decreasing order of priority:
  /// if verbosity is specified from command line, e.g. `-v` or `-vv`, use that
  /// if a `RUST_LOG` env var is set, use that
  /// else, use INFO
  pub fn log_level(&self) -> LevelFilter {
    if self.verbosity > 0 {
      match self.verbosity {
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
      }
    } else if let Ok(s) = std::env::var("RUST_LOG") {
      s.parse().expect("RUST_LOG environment invalid value")
    } else {
      LevelFilter::Info
    }
  }
}

// option to use subcommand args
pub(crate) mod subcommand {
  use super::*;

  // https://docs.rs/clap/latest/clap/
  /// CLI parser
  #[derive(Parser, Debug)]
  #[command(name = "tk")]
  #[command(bin_name = "tk")]
  #[clap(about = "tk does things")]
  pub enum SubcommandArgs {
    First(MyArgs),
  }

  impl SubcommandArgs {
    delegate::delegate! {
      to match self {
        SubcommandArgs::First(subcommand) => subcommand,
      } {
        pub fn log_level(&self) -> LevelFilter;
      }
    }
  }
use anyhow::{anyhow, Result};
use clap::Parser;
use log::{trace, LevelFilter};

use crate::{cli::MyArgs, error::MyError};

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

mod subcommand {
  use super::*;
  use crate::cli::subcommand::SubcommandArgs;

  /// Set up crate logging and environment variables.
  pub(crate) fn setup() -> Result<SubcommandArgs, MyError> {
    dotenv::dotenv().ok();
    // init_tracing();
    let args = SubcommandArgs::parse();
    if std::env::var("DOTENV_OK").is_ok() {
      trace!("loaded dotenv");
    } else {
      return Err(anyhow!("failed to load dotenv").into());
    }
    env_logger::builder().filter_level(args.log_level()).build();

    Ok(args)
  }
}
