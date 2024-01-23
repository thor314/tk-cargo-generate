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
  pub name:  String,
  /// Number of times to greet
  #[arg(short, long, default_value_t = 1)]
  pub count: usize,
}
