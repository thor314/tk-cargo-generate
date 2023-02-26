use anyhow::Result;
// use tracing::instrument;
// use tracing_subscriber::filter::{EnvFilter, LevelFilter};


/// Set up crate logging and environment variables.
pub(crate) fn setup() -> Result<()> {
  // source .env file
  dotenv::dotenv().ok();
  // init logger
  // init_tracing();
  env_logger::init();
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
