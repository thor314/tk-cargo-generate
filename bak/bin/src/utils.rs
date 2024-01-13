use anyhow::Result;
// use tracing::instrument;
// use tracing_subscriber::filter::{EnvFilter, LevelFilter};

/// Set up crate logging and environment variables.
pub(crate) fn setup() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();
  // init_tracing();
  // let args = TkCLI::parse();
  // Ok(args)
  Ok(())
}

// /// Set up the tracing filter level using the env value, or else set it here. Reads RUST_LOG.
// /// TRACE < DEBUG < INFO < WARN < ERROR
// #[instrument]
// pub(crate) fn init_tracing() {
//   let filter = LevelFilter::INFO.into();
//   // set level to RUST_LOG env variable, or else INFO
//   let filter = EnvFilter::builder().with_default_directive(filter).from_env_lossy();
//   //  .with_level(false) // don't include levels in formatted output
//   //  .with_target(false) // don't include targets
//   //  .with_thread_ids(true) // include the thread ID of the current thread
//   //  .with_thread_names(true) // include the name of the current thread
//   //  .compact(); // use the `Compact` formatting style.
//   tracing_subscriber::fmt().with_env_filter(filter).init();
// }

// /// Subcommand-enabled CLI parser
// #[derive(Parser, Debug)]
// #[command(name = "tk")]
// #[command(bin_name = "tk")]
// pub enum TkCLI {
//   Testing(TestArgs),
// }

// /// Simple program to greet a person
// #[derive(Debug, clap::Args)]
// #[command(author, version, about, long_about = None)]
// pub struct TestArgs {
//   /// Name of the person to greet
//   #[arg(short, long)]
//   pub name: String,

//   /// Number of times to greet
//   #[arg(short, long, default_value_t = 1)]
//   pub count: u8,
// }
