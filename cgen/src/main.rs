//! A batteries-included binary template.

// TODO: remove these when ready
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use anyhow::Result;
use clap::Parser;
{%- if sync == "async" %}
use futures::{
  channel::mpsc,
  executor::{self, ThreadPool},
  StreamExt,
};
{%- endif %}
use utils::MyError;
use validator::{Validate, ValidationError};

mod utils;

{%- if sync == "async" %}
#[tokio::main]
async fn main() -> Result<()> {
{%- else %}
fn main() -> Result<()> {
{%- endif %}
  let context = utils::setup()?;
  if std::env::var("DOTENV_OK").is_ok() {
    {%- if sync == "async" %}
    tracing::info!("Hello, {}!", context.args.name);
    #[cfg(feature = "some_feature")]
    tracing::debug!("and build info: {:#?}", context.s);
    let pool = ThreadPool::new().expect("Failed to build pool");
    let (tx, rx) = mpsc::unbounded::<i32>();
    let fut_values = async {
      let fut_tx_result =
        async move { (0..100).for_each(|v| tx.unbounded_send(v).expect("failed send")) };
      pool.spawn_ok(fut_tx_result); // spawn generated future
      let fut_values = rx.map(|v| v * 2).collect();
      // Use the executor provided to this async block to wait for the future to complete.
      fut_values.await
    };
    let values: Vec<i32> = executor::block_on(fut_values); // consume the future
    println!("Values={:?}", values);
    {%- else %}
    log::info!("Hello, {}!", context.args.name);
    #[cfg(feature = "some_feature")]
    log::debug!("and build info: {:#?}", context.s);
    {%- endif %}
  }
  Ok(())
}
