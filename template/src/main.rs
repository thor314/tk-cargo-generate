#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

use error::MyError;

mod error;
#[cfg(test)] mod tests;
mod utils;
{% if cli %} mod cli; {% endif %}
{% if async -%} use tracing::info;
{% else -%} use log::info;
{% endif %}

fn main() -> Result<(), MyError> {
  {% if cli %} let _cli = {% endif %}
  utils::setup()?;
  info!("hello thor"); 


  Ok(())
}