//! A batteries-included binary template.

// remove these when ready
#![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(dead_code)]
{%- if async %}
use async_std::{channel, prelude::*};
use futures_lite::prelude::*;
{%- endif %}
use utils::MyError;

#[cfg(test)] mod tests;
mod utils;

{%- if async %}
#[async_std::main]
async fn main() -> Result<(), MyError> {
{%- else %}
fn main() -> Result<()> {
{%- endif %}
  let context = utils::setup()?;
  if std::env::var("DOTENV_OK").is_ok() {
    {%- if async %}
    tracing::info!("Hello, {}!", context.args.name);
    tracing::debug!("and build info: {:#?}", context.s);
    let (tx, rx) = channel::unbounded::<i32>();
    let _ = async {
      (0..100).for_each(|v| {
        let _ = tx.send(v);
      })
    }
    .await;

    let mut values = vec![];
    while let Ok(v) = rx.try_recv() {
      values.push(v);
    }

    println!("Values={:?}", values);
    {%- else %}
    log::info!("Hello, {}!", context.args.name);
    log::debug!("and build info: {:#?}", context.s);
    {%- endif %}
  }
  Ok(())
}
