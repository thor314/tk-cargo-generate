use clap::{ArgAction, Args, Parser};
use log::{debug, trace, LevelFilter};

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
  pub fn handle(&self) {
    trace!("handling MyArgs");
  }

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

    pub fn handle(&self) {
      trace!("handling subcommands...");
      match self {
        SubcommandArgs::First(c) => c.handle(),
      }
    }
  }
}
