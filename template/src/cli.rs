use clap::{ArgAction, Args, Parser};

// args mode
#[derive(Parser, Debug)]
#[command(name = "tk")]
#[command(bin_name = "tk")]
// #[clap(about = "tk does things")]
pub struct Args {
  /// Set the verbosity. Use -v for DEBUG, -vv for TRACE.
  // Count will count the number -v flags. e.g. -v -v and -vv are both 2.
  #[arg(long = "verbose", short = 'v', action = ArgAction::Count)]
  verbosity: u8,
}
// todo: add this to log level parser
// .with_max_level(match args.verbosity {
//   0 => Level::INFO,
//   1 => Level::DEBUG,
//   _ => Level::TRACE,
// }).with_target(args.verbosity > 0)

// subcommand mode
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
  #[arg(short = 'n', long)]
  pub name:  String,
  /// Number of times to greet
  #[arg(short = 'c', long, default_value_t = 1)]
  pub count: usize,
}
