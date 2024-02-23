//! https://docs.rs/clap/latest/clap/

use clap::{ArgAction, Args, Parser, Subcommand};
{% if async -%}
use tracing::trace;
use tracing_subscriber::filter::{LevelFilter, EnvFilter};
{% else -%}
use log::{trace, LevelFilter};
{% endif -%}

/// The subcommand handler.
/// If no subcommand is provided, the handler will short to the logic for the default command.
///
/// This struct probably doesn't need to change, make changes to `Subcommands` and the individual
/// subcommands instead.
#[derive(Parser, Debug)]
#[command(name = "{{crate_name}}")]
#[command(bin_name = "{{crate_name}}")]
#[clap(about = "{{crate_name}} cli")]
#[command(author, version)]
#[command(propagate_version = true)]
pub struct MyCli {
  #[command(subcommand)]
  subcommands:   Option<Subcommands>,
  /// Set the verbosity. Use -v for DEBUG, -vv for TRACE. None for INFO.
  #[arg(long = "verbose", short = 'v', action = ArgAction::Count)]
  pub verbosity: u8,
}

impl MyCli {
  pub fn handle(&self) {
    match &self.subcommands {
      Some(subcommands) => subcommands.handle(),
      None => self.handle_default(),
    }
  }

  /// in decreasing order of priority:
  /// if verbosity is specified from command line, e.g. `-v` or `-vv`, use that
  /// if a `RUST_LOG` env var is set, use that
  /// else, use INFO
  pub fn log_level(&self) -> LevelFilter {
    if self.verbosity > 0 {
      match self.verbosity {
        1 =>  {% if async %} LevelFilter::DEBUG, {% else %} LevelFilter::Debug, {% endif %}
        _ =>  {% if async %} LevelFilter::TRACE, {% else %} LevelFilter::Trace, {% endif %}
      }
    } else if let Ok(s) = std::env::var("RUST_LOG") {
      s.parse().expect("RUST_LOG environment invalid value")
    } else {
      {% if async %} LevelFilter::INFO {% else %} LevelFilter::Info {% endif %}
    }
  }

  /// The default command: what to do if no subcommand is provided 
  fn handle_default(&self) { trace!("handle default") }
}

/// CLI parser with subcommands
/// The subcommands for this CLI.
/// Add subcommands as demonstrated.
#[derive(Debug, Subcommand)]
enum Subcommands {
  SayHello(SayHello),
}

//
impl Subcommands {
  /// delegate handling to each subcommand
  pub fn handle(&self) {
    trace!("handling subcommands...");
    match self {
      Subcommands::SayHello(c) => c.handle(),
    }
  }
}

// test with: 
// cargo run -- say-hello --hello
/// An example subcommand
#[derive(Parser, Debug)]
struct SayHello {
  /// example
  #[arg(long = "hello")]
  pub hello_world: bool,
}

impl SayHello {
  pub fn handle(&self) {
    if self.hello_world {
      println!("hello world!");
    }
  }
}

