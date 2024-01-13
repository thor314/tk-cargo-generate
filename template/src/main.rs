#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use error::MyError;
use log::{error, info};

mod error;
#[cfg(test)] mod tests;
mod utils;

fn main() -> Result<(), MyError> {
  {% if cli %} 
  let _cli =
  {%- endif %} 
  utils::setup()?;

  info!("hello thor");
  Ok(())
}

